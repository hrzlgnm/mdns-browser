// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    net::IpAddr,
    sync::{Arc, Mutex},
};
use tauri::State;
use tauri_plugin_log::LogTarget;

type SharedServiceDaemon = Arc<Mutex<ServiceDaemon>>;

struct Daemon {
    shared: SharedServiceDaemon,
}

fn get_shared_daemon() -> SharedServiceDaemon {
    let daemon = ServiceDaemon::new().expect("Failed to create daemon");
    Arc::new(Mutex::new(daemon))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ResolvedService {
    instance_name: String,
    hostname: String,
    port: u16,
    addresses: HashSet<IpAddr>,
}

#[tauri::command]
fn resolve_service(service_type: String, state: State<Daemon>) -> Vec<ResolvedService> {
    log::info!("Resolving {}", service_type);
    let mdns = state.shared.lock().unwrap();
    let mut service_type = service_type;
    if !service_type.ends_with(".local.") {
        service_type.push_str(".local.");
    }
    let receiver = mdns
        .browse(service_type.as_str())
        .expect("Failed to browse");
    let mut result = vec![];
    let mut done = false;
    while let Ok(event) = receiver.recv() {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                result.push(ResolvedService {
                    instance_name: info.get_fullname().into(),
                    hostname: info.get_hostname().into(),
                    port: info.get_port(),
                    addresses: info.get_addresses().clone(),
                });
            }
            ServiceEvent::SearchStarted(_) => {
                if done {
                    let _ = mdns.stop_browse(service_type.as_str());
                }
                done = true;
            }
            ServiceEvent::SearchStopped(_) => {
                break;
            }
            _ => {}
        }
    }
    result
}

#[tauri::command]
fn enum_service_types(state: State<Daemon>) -> Vec<String> {
    let mut found = vec![];
    if let Ok(mdns) = state.shared.lock() {
        let meta_service = "_services._dns-sd._udp.local.";
        let receiver = mdns.browse(meta_service).expect("Failed to browse");
        let mut search_done = false;
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceFound(service_type, full_name) => {
                    if !full_name.starts_with(&service_type) {
                        found.push(full_name);
                    }
                }
                ServiceEvent::SearchStarted(_service) => {
                    if search_done {
                        let _ = mdns.stop_browse(meta_service);
                    }
                    search_done = true;
                }
                ServiceEvent::SearchStopped(_service) => {
                    break;
                }
                _ => {}
            }
        }
        let mr = mdns.get_metrics().expect("Failed to get metrics");
        if let Ok(metrics) = mr.recv() {
            log::debug!("Metrics {:#?}", metrics);
        }
        found.sort();
        log::debug!("Found service types: {:#?}", found);
    }
    found
}

#[cfg(target_os = "linux")]
fn platform_setup() {
    let sessiopn_type_key = "XDG_SESSION_TYPE";
    match std::env::var(sessiopn_type_key) {
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

fn main() {
    platform_setup();
    tauri::Builder::default()
        .manage(Daemon {
            shared: get_shared_daemon(),
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .level(LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            enum_service_types,
            resolve_service
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
