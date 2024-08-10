// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use serde::Serialize;
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};
use tauri::Emitter;
use tauri::{Manager, State, Window};
use tauri_plugin_log::{Target, TargetKind};

type SharedServiceDaemon = Arc<Mutex<ServiceDaemon>>;

struct MdnsState {
    daemon: SharedServiceDaemon,
    running_browsers: Arc<Mutex<Vec<String>>>,
}

impl MdnsState {
    fn new() -> Self {
        Self {
            daemon: get_shared_daemon(),
            running_browsers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

fn get_shared_daemon() -> SharedServiceDaemon {
    let daemon = ServiceDaemon::new().expect("Failed to create daemon");
    Arc::new(Mutex::new(daemon))
}

#[derive(Serialize, Clone, Debug)]
struct TxtRecord {
    key: String,
    val: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ResolvedService {
    instance_name: String,
    hostname: String,
    port: u16,
    pub addresses: Vec<IpAddr>,
    subtype: Option<String>,
    txt: Vec<TxtRecord>,
    updated_at_ms: u64,
}

fn timestamp_millis() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();

    since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_millis())
}

fn string_with_control_characters_escaped(input: String) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_control() {
                format!(r"\u{:04x}", ch as u32)
            } else {
                ch.to_string()
            }
        })
        .collect()
}

fn bytes_option_to_string_option_with_escaping(maybe_bytes: Option<&[u8]>) -> Option<String> {
    maybe_bytes.map(|bytes| match String::from_utf8(bytes.to_vec()) {
        Ok(utf8_string) => {
            let result = string_with_control_characters_escaped(utf8_string);
            log::debug!("bytes {:#?} -> str {}", bytes, result);

            result
        }
        Err(_) => byte_array_hexlified(bytes),
    })
}

fn byte_array_hexlified(byte_array: &[u8]) -> String {
    byte_array
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}

impl From<&ServiceInfo> for ResolvedService {
    fn from(info: &ServiceInfo) -> ResolvedService {
        let mut sorted_addresses: Vec<IpAddr> = info.get_addresses().clone().drain().collect();
        sorted_addresses.sort();
        let mut sorted_txt: Vec<TxtRecord> = info
            .get_properties()
            .iter()
            .map(|r| {
                log::debug!("txt prop {:#?}", r.val());
                TxtRecord {
                    key: r.key().into(),
                    val: bytes_option_to_string_option_with_escaping(r.val()),
                }
            })
            .collect();
        sorted_txt.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
        ResolvedService {
            instance_name: info.get_fullname().into(),
            hostname: info.get_hostname().into(),
            port: info.get_port(),
            addresses: sorted_addresses,
            subtype: info.get_subtype().clone(),
            txt: sorted_txt,
            updated_at_ms: timestamp_millis(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct MetricsEvent {
    metrics: HashMap<String, i64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ServiceResolvedEvent {
    service: ResolvedService,
}

#[derive(Serialize, Clone, Debug)]
pub struct SearchStartedEvent {
    service_type: String,
}

type SearchStoppedEvent = SearchStartedEvent;

#[derive(Serialize, Clone, Debug)]
pub struct ServiceRemovedEvent {
    instance_name: String,
    at_ms: u64,
}

type ServiceFoundEvent = ServiceRemovedEvent;
type ServiceTypeFoundEvent = SearchStartedEvent;

#[tauri::command]
fn browse_types(window: Window, state: State<MdnsState>) {
    if let Ok(mdns) = state.daemon.lock() {
        let mdns_for_thread = mdns.clone();
        std::thread::spawn(move || {
            let meta_service = "_services._dns-sd._udp.local.";
            let receiver = mdns_for_thread
                .browse(meta_service)
                .expect("Failed to browse");
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceFound(service_type, full_name) => {
                        if !full_name.starts_with(&service_type) {
                            window
                                .emit(
                                    "service-type-found",
                                    &ServiceTypeFoundEvent {
                                        service_type: full_name,
                                    },
                                )
                                .expect("To emit");
                        }
                    }
                    ServiceEvent::SearchStopped(service_type) => {
                        if service_type == meta_service {
                            break;
                        }
                    }
                    _ => (),
                }
            }
            log::debug!("Browse type thread ending.");
        });
    }
}

#[tauri::command]
fn stop_browse(service_type: String, state: State<MdnsState>) {
    if service_type.is_empty() {
        return;
    }
    if let Ok(mdns) = state.daemon.lock() {
        if let Ok(mut running_browsers) = state.running_browsers.lock() {
            if running_browsers.contains(&service_type) {
                mdns.stop_browse(service_type.as_str())
                    .expect("To stop browsing");
                running_browsers.retain(|s| s != &service_type);
            }
        }
    }
}

#[tauri::command]
fn browse(service_type: String, window: Window, state: State<MdnsState>) {
    if service_type.is_empty() {
        return;
    }
    if let Ok(mdns) = state.daemon.lock() {
        if let Ok(mut running_browsers) = state.running_browsers.lock() {
            if !running_browsers.contains(&service_type) {
                running_browsers.push(service_type.clone());
                let receiver = mdns.browse(service_type.as_str()).expect("To browse");
                std::thread::spawn(move || {
                    while let Ok(event) = receiver.recv() {
                        match event {
                            ServiceEvent::ServiceFound(_service_type, instance_name) => {
                                window
                                    .emit(
                                        "service-found",
                                        &ServiceFoundEvent {
                                            instance_name,
                                            at_ms: timestamp_millis(),
                                        },
                                    )
                                    .expect("To emit");
                            }
                            ServiceEvent::SearchStarted(service_type) => {
                                window
                                    .emit("search-started", &SearchStartedEvent { service_type })
                                    .expect("to emit");
                            }
                            ServiceEvent::ServiceResolved(info) => {
                                window
                                    .emit(
                                        "service-resolved",
                                        &ServiceResolvedEvent {
                                            service: ResolvedService::from(&info),
                                        },
                                    )
                                    .expect("To emit");
                            }
                            ServiceEvent::ServiceRemoved(_service_type, instance_name) => {
                                window
                                    .emit(
                                        "service-removed",
                                        &ServiceRemovedEvent {
                                            instance_name,
                                            at_ms: timestamp_millis(),
                                        },
                                    )
                                    .expect("To emit");
                            }
                            ServiceEvent::SearchStopped(service_type) => {
                                window
                                    .emit("search-stopped", &SearchStoppedEvent { service_type })
                                    .expect("To emit");
                                break;
                            }
                        }
                    }
                    log::debug!("Browse thread for {} ending.", &service_type);
                });
            }
        }
    }
}

const METRIC_SEND_INTERVAL: Duration = Duration::from_secs(10);

#[tauri::command]
fn send_metrics(window: Window, state: State<MdnsState>) {
    if let Ok(mdns) = state.daemon.lock() {
        let mdns_for_thread = mdns.clone();
        std::thread::spawn(move || loop {
            if let Ok(metrics_receiver) = mdns_for_thread.get_metrics() {
                if let Ok(metrics) = metrics_receiver.recv() {
                    window
                        .emit("metrics", &MetricsEvent { metrics })
                        .expect("To emit");
                }
            }
            std::thread::sleep(METRIC_SEND_INTERVAL);
        });
    }
}

#[cfg(target_os = "linux")]
fn platform_setup() {
    let session_type_key = "XDG_SESSION_TYPE";
    match std::env::var(session_type_key) {
        Ok(val) => {
            if val == "x11" {
                println!(
                    "Setting WEBKIT_DISABLE_COMPOSITING_MODE=1 to workaround rendering issues with x11 session"
                );
                std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1")
            }
        }
        Err(_e) => {}
    }
}

#[cfg(not(target_os = "linux"))]
fn platform_setup() {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    platform_setup();
    tauri::Builder::default()
        .manage(MdnsState::new())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
            let splashscreen_window = app.get_webview_window("splashscreen").unwrap();
            let main_window = app.get_webview_window("main").unwrap();
            let ver = app.config().version.clone();
            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(3));
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
                main_window
                    .set_title(
                        format!("mDNS-Browser v{}", ver.unwrap_or(String::from("Unknown")))
                            .as_str(),
                    )
                    .expect("title to be set");
            });
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .level(LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            browse,
            browse_types,
            send_metrics,
            stop_browse
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
