// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use log::LevelFilter;
use mdns_sd::{ServiceDaemon, ServiceEvent};
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
                        found.push(full_name.replace(".local.", ""));
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
fn setup_hook() {
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
fn setup_hook() {}

fn main() {
    setup_hook();
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
        .invoke_handler(tauri::generate_handler![enum_service_types])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
