// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use if_addrs::{get_if_addrs, IfAddr, Interface};
use log::LevelFilter;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use serde::Serialize;
use std::{
    collections::HashMap,
    collections::HashSet,
    net::IpAddr,
    sync::{Arc, Mutex},
};
use tauri::Manager;
use tauri::State;
use tauri_plugin_log::LogTarget;

type SharedServiceDaemon = Arc<Mutex<ServiceDaemon>>;

struct MdnsState {
    shared: SharedServiceDaemon,
    enabled_interfaces: Arc<Mutex<Vec<Interface>>>,
}

impl MdnsState {
    fn new() -> Self {
        Self {
            shared: get_shared_daemon(),
            enabled_interfaces: Arc::new(Mutex::new(get_all_interfaces_except_loopback())),
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
    val: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ResolvedService {
    instance_name: String,
    hostname: String,
    port: u16,
    pub addresses: Vec<IpAddr>,
    subtype: Option<String>,
    txt: Vec<TxtRecord>,
}

#[tauri::command]
fn resolve_service(service_type: String, state: State<MdnsState>) -> Vec<ResolvedService> {
    log::info!("Resolving {}", service_type);
    let mdns = state.shared.lock().unwrap();
    let mut service_type = service_type;
    if !service_type.ends_with(".local.") {
        service_type.push_str(".local.");
    }
    let receiver = mdns
        .browse(service_type.as_str())
        .expect("Failed to browse");
    let mut result = HashMap::new();
    let mut done = false;
    while let Ok(event) = receiver.recv() {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                let mut sorted_addresses: Vec<IpAddr> =
                    info.get_addresses().clone().drain().collect();
                sorted_addresses.sort();
                let mut sorted_txt: Vec<TxtRecord> = info
                    .get_properties()
                    .iter()
                    .map(|r| TxtRecord {
                        key: r.key().into(),
                        val: r.val_str().into(),
                    })
                    .collect();
                sorted_txt.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
                result.insert(
                    info.get_fullname().to_string(),
                    ResolvedService {
                        instance_name: info.get_fullname().into(),
                        hostname: info.get_hostname().into(),
                        port: info.get_port(),
                        addresses: sorted_addresses,
                        subtype: info.get_subtype().clone(),
                        txt: sorted_txt,
                    },
                );
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

    filter_resolved_service_by_interfaces_addresses(
        result.values().cloned().collect(),
        state.enabled_interfaces.lock().unwrap().clone(),
    )
}

fn valid_ip_on_interface(addr: &IpAddr, interface: &Interface) -> bool {
    match (addr, &interface.addr) {
        (IpAddr::V4(addr), IfAddr::V4(interface_address)) => {
            let netmask = u32::from(interface_address.netmask);
            let interface_net = u32::from(interface_address.ip) & netmask;
            let addr_net = u32::from(*addr) & netmask;
            addr_net == interface_net
        }
        (IpAddr::V6(addr), IfAddr::V6(interface_address)) => {
            let netmask = u128::from(interface_address.netmask);
            let interface_net = u128::from(interface_address.ip) & netmask;
            let addr_net = u128::from(*addr) & netmask;
            addr_net == interface_net
        }
        _ => false,
    }
}

fn get_addresses_on_interface(addr: &Vec<IpAddr>, interface: &Interface) -> Vec<IpAddr> {
    addr.iter()
        .filter(|a| valid_ip_on_interface(a, interface))
        .copied()
        .collect()
}

fn filter_resolved_service_by_interfaces_addresses(
    resolved_services: Vec<ResolvedService>,
    interfaces: Vec<Interface>,
) -> Vec<ResolvedService> {
    let mut result = Vec::<ResolvedService>::new();
    for resolved_service in resolved_services.iter() {
        let mut unique_addresses = HashSet::<IpAddr>::new();
        for interface in interfaces.iter() {
            unique_addresses.extend(get_addresses_on_interface(
                &resolved_service.addresses,
                &interface,
            ));
        }
        let mut addresses = unique_addresses.into_iter().collect::<Vec<_>>();
        if !addresses.is_empty() {
            addresses.sort();
            result.push(ResolvedService {
                instance_name: resolved_service.instance_name.clone(),
                hostname: resolved_service.hostname.clone(),
                port: resolved_service.port,
                addresses,
                subtype: resolved_service.subtype.clone(),
                txt: resolved_service.txt.clone(),
            });
        }
    }
    result
}

#[tauri::command]
fn enum_service_types(state: State<MdnsState>) -> Vec<String> {
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
        log::debug!("Found service types: {:?}", found);
    }
    found
}

fn get_all_interfaces_except_loopback() -> Vec<Interface> {
    let interface_addresses = get_if_addrs().unwrap();

    // if_addrs unfortunately shows the GUID of the interface as name,
    // so we work around here by using network_interface in addition to get user friendly names
    let network_interfaces = NetworkInterface::show().unwrap();
    let mut index_to_name = HashMap::new();
    for network_interface in network_interfaces.iter() {
        index_to_name.insert(network_interface.index, network_interface.name.clone());
    }

    interface_addresses
        .into_iter()
        .filter(|itf| !itf.is_loopback())
        .map(|itf| Interface {
            name: index_to_name.get(&itf.index.unwrap()).unwrap().clone(),
            addr: itf.addr,
            index: itf.index,
        })
        .collect()
}

fn get_all_interface_names_except_loopback() -> Vec<String> {
    let interface_addresses = get_all_interfaces_except_loopback();

    interface_addresses
        .into_iter()
        .map(|interface| interface.name)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

#[tauri::command]
fn get_interfaces() -> Vec<String> {
    let mut interface_names = get_all_interface_names_except_loopback();
    interface_names.sort();

    interface_names
}

#[tauri::command]
fn set_interfaces(interfaces: Vec<String>, state: State<MdnsState>) {
    let interface_names = get_all_interface_names_except_loopback();

    let enabled_interface_names = interface_names
        .into_iter()
        .filter(|name| interfaces.contains(name))
        .collect::<Vec<_>>();
    let enabled_interfaces = get_all_interfaces_except_loopback()
        .into_iter()
        .filter(|interface| enabled_interface_names.contains(&interface.name))
        .collect::<Vec<_>>();
    *state.enabled_interfaces.lock().unwrap() = enabled_interfaces;
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
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let ver = app.config().package.version.clone();
            main_window
                .set_title(
                    format!("mDNS-Browser v{}", ver.unwrap_or(String::from("Unknown"))).as_str(),
                )
                .expect("title to be set");
            Ok(())
        })
        .manage(MdnsState::new())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .level(LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            enum_service_types,
            resolve_service,
            get_interfaces,
            set_interfaces
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
