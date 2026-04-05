// Copyright 2024-2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

#[cfg(desktop)]
use clap::builder::TypedValueParser as _;
#[cfg(desktop)]
use clap::Parser;
use mdns_sd::{IfKind, ServiceDaemon, ServiceEvent};
use models::check_service_type_fully_qualified;
use models::*;
use shared_constants::{
    INTERFACES_CAN_BROWSE_CHECK_INTERVAL, MDNS_SD_IP_CHECK_INTERVAL, MDNS_SD_META_SERVICE,
    METRICS_CHECK_INTERVAL, VERIFY_TIMEOUT,
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    net::IpAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};
use tauri::{AppHandle, Emitter, Manager, State, Theme, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_opener::OpenerExt;

type SharedServiceDaemon = Arc<Mutex<ServiceDaemon>>;

struct ManagedState {
    daemon: SharedServiceDaemon,
    queriers: Arc<Mutex<HashSet<String>>>,
    metrics_subscribed: AtomicBool,
    can_browse_subscribed: AtomicBool,
    ipv4_enabled: AtomicBool,
    ipv6_enabled: AtomicBool,
    #[cfg(desktop)]
    dev_tools_enabled: bool,
}

impl ManagedState {
    #[cfg(desktop)]
    fn new(dev_tools_requested: bool) -> Self {
        Self {
            daemon: initialize_shared_daemon(),
            queriers: Arc::new(Mutex::new(HashSet::new())),
            metrics_subscribed: AtomicBool::new(false),
            can_browse_subscribed: AtomicBool::new(false),
            ipv4_enabled: AtomicBool::new(true),
            ipv6_enabled: AtomicBool::new(true),
            dev_tools_enabled: dev_tools_requested,
        }
    }

    #[cfg(mobile)]
    fn new() -> Self {
        Self {
            daemon: initialize_shared_daemon(),
            queriers: Arc::new(Mutex::new(HashSet::new())),
            metrics_subscribed: AtomicBool::new(false),
            can_browse_subscribed: AtomicBool::new(false),
            ipv4_enabled: AtomicBool::new(true),
            ipv6_enabled: AtomicBool::new(true),
        }
    }
}

fn initialize_shared_daemon() -> SharedServiceDaemon {
    let daemon = ServiceDaemon::new().expect("Failed to create daemon");
    if let Err(err) = daemon.set_ip_check_interval(MDNS_SD_IP_CHECK_INTERVAL.as_secs() as u32) {
        log::warn!("Failed to set ip check interval: {err:?}, continuing anyway");
    }
    if let Err(err) = daemon.disable_interface(enumerate_mdns_incapable_interfaces()) {
        log::warn!("Failed to disable interface: {err:?}, continuing anyway");
    }
    Arc::new(Mutex::new(daemon))
}

fn convert_interface_id(id: &mdns_sd::InterfaceId) -> InterfaceScope {
    InterfaceScope {
        name: id.name.clone(),
        index: id.index,
    }
}

fn convert_to_scoped_addr(host_ip: &mdns_sd::ScopedIp) -> ScopedAddr {
    match host_ip {
        mdns_sd::ScopedIp::V4(host_ip_v4) => {
            let interfaces: BTreeSet<InterfaceScope> = host_ip_v4
                .interface_ids()
                .iter()
                .map(convert_interface_id)
                .collect();
            ScopedAddr {
                addr: host_ip.to_ip_addr(),
                interfaces,
                scope_id: None,
            }
        }
        mdns_sd::ScopedIp::V6(host_ip_v6) => {
            let interface = convert_interface_id(host_ip_v6.scope_id());
            let ip_addr = host_ip.to_ip_addr();
            let is_link_local = matches!(ip_addr, IpAddr::V6(v6) if v6.is_unicast_link_local());
            let scope_id = if is_link_local {
                #[cfg(windows)]
                {
                    Some(interface.index.to_string())
                }
                #[cfg(not(windows))]
                {
                    Some(interface.name.clone())
                }
            } else {
                None
            };
            let interfaces = BTreeSet::from([interface]);
            ScopedAddr {
                addr: ip_addr,
                interfaces,
                scope_id,
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod convert_to_scoped_addr_tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_convert_to_scoped_addr_ipv4() {
        use mdns_sd::{InterfaceId, ScopedIp, ScopedIpV4};

        let ipv4_addr = Ipv4Addr::new(192, 168, 1, 1);
        let interface_id = InterfaceId {
            name: "eth0".to_string(),
            index: 2,
        };
        let scoped_ip = ScopedIp::V4(ScopedIpV4::new(ipv4_addr, interface_id));

        let result = convert_to_scoped_addr(&scoped_ip);

        assert_eq!(result.addr, IpAddr::V4(ipv4_addr));
        assert!(!result.interfaces.is_empty());
    }

    #[test]
    fn test_convert_to_scoped_addr_ipv6() {
        use mdns_sd::ScopedIp;

        let ipv6_addr = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        let scoped_ip = ScopedIp::from(IpAddr::V6(ipv6_addr));

        let result = convert_to_scoped_addr(&scoped_ip);

        assert_eq!(result.addr, IpAddr::V6(ipv6_addr));
    }
}

fn from_resolved_service(resolved: &mdns_sd::ResolvedService) -> ResolvedService {
    let addresses: Vec<ScopedAddr> = resolved
        .addresses
        .iter()
        .map(convert_to_scoped_addr)
        .collect();

    let mut consolidated: Vec<ScopedAddr> = Vec::new();
    for addr in addresses {
        let is_ipv6_link_local = matches!(addr.addr, IpAddr::V6(v6) if v6.is_unicast_link_local());
        if is_ipv6_link_local {
            consolidated.push(addr);
        } else if let Some(existing) = consolidated.iter_mut().find(|a| a.addr == addr.addr) {
            existing.interfaces.extend(addr.interfaces);
        } else {
            consolidated.push(addr);
        }
    }
    consolidated.sort();

    let mut sorted_txt: Vec<TxtRecord> = resolved
        .txt_properties
        .iter()
        .map(|r| TxtRecord {
            key: r.key().into(),
            val: bytes_option_to_string_option_with_escaping(r.val()),
        })
        .collect();
    sorted_txt.sort_by(|a, b| a.key.cmp(&b.key));
    ResolvedService {
        instance_fullname: resolved.fullname.clone(),
        service_type: resolved.ty_domain.clone(),
        hostname: resolved.host.clone(),
        port: resolved.port,
        addresses: consolidated,
        subtype: resolved.sub_ty_domain.clone(),
        txt: sorted_txt,
        updated_at_micros: timestamp_micros(),
        dead: false,
    }
}

/// Emits an event to the window.
///
/// This helper centralizes event emission and handles any errors internally.
/// If window.emit returns an error, it is logged and not propagated.
/// Use this helper to avoid repetitive error handling.
fn emit_event<T>(window: &Window, event: &str, payload: &T)
where
    T: serde::Serialize + std::fmt::Debug,
{
    log::trace!("Emitting event: {event} with payload: {payload:#?}");
    if let Err(e) = window.emit(event, payload) {
        log::error!("Failed to emit {event} event: {e:?}");
    }
}

#[tauri::command]
fn browse_types(window: Window, state: State<ManagedState>) -> Result<(), String> {
    let daemon = state
        .daemon
        .lock()
        .map_err(|e| format!("Failed to lock daemon: {e:?}"))?;

    daemon
        .stop_browse(MDNS_SD_META_SERVICE)
        .map_err(|e| format!("Failed to stop browsing for {MDNS_SD_META_SERVICE}: {e:?}"))?;

    let daemon = daemon.clone();
    tauri::async_runtime::spawn(async move {
        let receiver = match daemon.browse(MDNS_SD_META_SERVICE) {
            Ok(receiver) => receiver,
            Err(e) => {
                log::error!("Failed to browse for service types: {e:?}");
                return;
            }
        };
        while let Ok(event) = receiver.recv_async().await {
            match event {
                ServiceEvent::ServiceFound(_service_type, full_name) => {
                    match check_service_type_fully_qualified(full_name.as_str()) {
                        Ok(MdnsLabelType::ServiceType) => {
                            emit_event(
                                &window,
                                "service-type-found",
                                &ServiceTypeFoundEvent {
                                    service_type: full_name,
                                },
                            );
                        }
                        Ok(MdnsLabelType::SubType) => {
                            log::debug!(
                                "Ignoring subtype `{full_name}` found during service type browsing"
                            );
                        }
                        Err(e) => {
                            log::debug!("Ignoring invalid service type `{full_name}`: {e}")
                        }
                    }
                }
                ServiceEvent::SearchStopped(service_type) => {
                    if service_type == MDNS_SD_META_SERVICE {
                        break;
                    }
                }
                _ => {}
            }
        }
    });
    Ok(())
}

#[tauri::command]
fn stop_browse(state: State<ManagedState>) -> Result<(), String> {
    let daemon = state
        .daemon
        .lock()
        .map_err(|e| format!("Failed to lock daemon: {e:?}"))?;
    let mut queriers = state
        .queriers
        .lock()
        .map_err(|e| format!("Failed to lock running queriers: {e:?}"))?;
    for ty_domain in queriers.iter() {
        if let Err(e) = daemon.stop_browse(ty_domain) {
            log::error!("Failed to stop browsing for {ty_domain}: {e:?}");
        }
    }

    queriers.clear();
    Ok(())
}

#[tauri::command]
fn verify(instance_fullname: String, state: State<ManagedState>) -> Result<(), String> {
    let daemon = state
        .daemon
        .lock()
        .map_err(|e| format!("Failed to lock daemon: {e:?}"))?;
    daemon
        .verify(instance_fullname.clone(), VERIFY_TIMEOUT)
        .map_err(|e| format!("Failed to verify {instance_fullname}: {e:?}"))?;
    Ok(())
}

#[tauri::command]
fn browse_many(service_types: Vec<String>, window: Window, state: State<ManagedState>) {
    for service_type in service_types {
        let daemon = match state.daemon.lock() {
            Ok(daemon) => daemon,
            Err(err) => {
                log::error!("Failed to lock daemon: {err:?}");
                continue;
            }
        };
        let mut queriers = match state.queriers.lock() {
            Ok(queriers) => queriers,
            Err(err) => {
                log::error!("Failed to lock running queriers: {err:?}");
                continue;
            }
        };
        if !queriers.insert(service_type.clone()) {
            continue;
        }
        let receiver = match daemon.browse(service_type.as_str()) {
            Ok(receiver) => receiver,
            Err(e) => {
                log::error!("Failed to start browsing for {service_type} browse: {e:?}",);
                continue;
            }
        };

        let window = window.clone();
        tauri::async_runtime::spawn(async move {
            while let Ok(event) = receiver.recv_async().await {
                match event {
                    ServiceEvent::ServiceResolved(resolved) => emit_event(
                        &window,
                        "service-resolved",
                        &ServiceResolvedEvent {
                            service: from_resolved_service(&resolved),
                        },
                    ),

                    ServiceEvent::ServiceRemoved(_service_type, instance_name) => {
                        emit_event(
                            &window,
                            "service-removed",
                            &ServiceRemovedEvent {
                                instance_name,
                                at_micros: timestamp_micros(),
                            },
                        );
                    }
                    ServiceEvent::SearchStopped(_service_type) => {
                        break;
                    }
                    _ => {}
                }
            }
        });
    }
}

#[cfg(not(windows))]
fn enumerate_mdns_incapable_interfaces() -> Vec<IfKind> {
    use pnet::datalink;
    let interfaces = datalink::interfaces();
    interfaces
        .iter()
        .filter_map(|interface| {
            // Skip loopback and point to point outright as those are disabled by
            // default.
            if interface.is_loopback() || interface.is_point_to_point() {
                return None;
            }
            // On android there are some `rmnet` = remote network virtual interfaces which are
            // cellular modem data interfaces. Those do not have a broadcast capability like
            // ethernet or wifi interface, so we disable those, too.
            // Sometimes there is also a dummy0 interface without a multicast capability which
            // we disable as well.
            let incapable = interface.ips.is_empty()
                || !interface.is_running()
                || !interface.is_multicast()
                || !interface.is_broadcast();
            if incapable {
                Some(IfKind::from(interface.name.as_str()))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(not(windows))]
#[cfg(test)]
mod tests {
    use super::*;
    use pnet::datalink;

    #[test]
    fn test_loopback_not_included_in_mdns_incapable_interfaces() {
        let result = enumerate_mdns_incapable_interfaces();
        // Gather actual loopback interface names on this system.
        let loopback_names: std::collections::HashSet<String> = {
            datalink::interfaces()
                .into_iter()
                .filter(|iface| iface.is_loopback())
                .map(|iface| iface.name)
                .collect()
        };
        assert!(
            !loopback_names.is_empty(),
            "No loopback interfaces detected on this host; test cannot validate exclusion"
        );
        let any_loopback_found = result.iter().any(|ifkind| match ifkind {
            IfKind::Name(name) => loopback_names.contains(name),
            _ => false,
        });
        assert!(
            !any_loopback_found,
            "Loopback interfaces {:?} should not be included in mdns-incapable interfaces",
            loopback_names
        );
    }
}

#[cfg(windows)]
fn enumerate_mdns_incapable_interfaces() -> Vec<IfKind> {
    use ipconfig::{IfType, OperStatus};

    if let Ok(adapters) = ipconfig::get_adapters() {
        adapters
            .iter()
            .filter_map(|adapter| {
                // Skip SoftwareLoopback, Tunnel, and Ppp interfaces as these
                // interface types are disabled by default.
                if matches!(
                    adapter.if_type(),
                    IfType::SoftwareLoopback | IfType::Tunnel | IfType::Ppp
                ) {
                    return None;
                }
                if adapter.ip_addresses().is_empty()
                    || adapter.oper_status() != OperStatus::IfOperStatusUp
                    || (adapter.if_type() != IfType::EthernetCsmacd
                        && adapter.if_type() != IfType::Ieee80211)
                {
                    Some(IfKind::from(adapter.friendly_name()))
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    }
}

#[cfg(windows)]
#[cfg(test)]
mod tests {
    use super::*;
    use ipconfig::IfType;

    #[test]
    fn test_loopback_not_included_in_mdns_incapable_interfaces() {
        let result = enumerate_mdns_incapable_interfaces();
        let loopback_names: std::collections::HashSet<String> = ipconfig::get_adapters()
            .map(|adapters| {
                adapters
                    .into_iter()
                    .filter(|a| a.if_type() == IfType::SoftwareLoopback)
                    .map(|a| a.friendly_name().to_string())
                    .collect()
            })
            .unwrap_or_default();
        assert!(
            !loopback_names.is_empty(),
            "No loopback interfaces detected on this host; test cannot validate exclusion"
        );
        let loopback_present = result.iter().any(|ifkind| match ifkind {
            IfKind::Name(name) => loopback_names.contains(name),
            _ => false,
        });
        assert!(
            !loopback_present,
            "Software loopback adapters {:?} should not be included in mdns-incapable interfaces",
            loopback_names
        );
    }
}

#[cfg(not(windows))]
fn has_mdns_capable_interfaces() -> bool {
    use pnet::datalink;
    let interfaces = datalink::interfaces();
    interfaces.iter().any(|interface| {
        !interface.ips.is_empty()
            && !interface.is_loopback()
            && !interface.is_point_to_point()
            && interface.is_multicast()
            && interface.is_broadcast()
            && interface.is_running()
    })
}

#[cfg(windows)]
fn has_mdns_capable_interfaces() -> bool {
    use ipconfig::{IfType, OperStatus};

    if let Ok(adapters) = ipconfig::get_adapters() {
        adapters.iter().any(|adapter| {
            !adapter.ip_addresses().is_empty()
                && adapter.oper_status() == OperStatus::IfOperStatusUp
                && (adapter.if_type() == IfType::EthernetCsmacd
                    || adapter.if_type() == IfType::Ieee80211)
        })
    } else {
        log::warn!("Unable to determine whether we have mDNS capable network adapters, assuming no network is present");
        false
    }
}

async fn poll_can_browse(window: Window) {
    let mut current = has_mdns_capable_interfaces();
    emit_event(
        &window,
        "can-browse-changed",
        &CanBrowseChangedEvent {
            can_browse: current,
        },
    );
    loop {
        tokio::time::sleep(INTERFACES_CAN_BROWSE_CHECK_INTERVAL).await;
        let new_value = has_mdns_capable_interfaces();
        if new_value != current {
            current = new_value;
            emit_event(
                &window,
                "can-browse-changed",
                &CanBrowseChangedEvent {
                    can_browse: current,
                },
            );
        }
    }
}

#[tauri::command]
fn subscribe_can_browse(window: Window, state: State<ManagedState>) {
    if state
        .can_browse_subscribed
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        tauri::async_runtime::spawn(poll_can_browse(window));
    } else {
        emit_event(
            &window,
            "can-browse-changed",
            &CanBrowseChangedEvent {
                can_browse: has_mdns_capable_interfaces(),
            },
        );
    }
}

#[tauri::command]
/// Subscribes to periodic mDNS daemon metrics updates and emits changes to the frontend.
///
/// Starts a background task that polls the mDNS daemon for metrics at regular intervals.
/// When metrics contents changes, emits a `"metrics-changed"` event to the Tauri window.
/// Ensures only one subscription is active at a time for the application window.
fn subscribe_metrics(window: Window, state: State<ManagedState>) {
    // Avoid multiple subscriptions when the frontend is reloaded.
    if state
        .metrics_subscribed
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        if let Ok(daemon) = state.daemon.lock() {
            let daemon = daemon.clone();
            let mut old_metrics = HashMap::new();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Ok(metrics_receiver) = daemon.get_metrics() {
                        if let Ok(metrics) = metrics_receiver.recv_async().await {
                            if old_metrics == metrics {
                                continue;
                            }
                            emit_event(
                                &window,
                                "metrics-changed",
                                &MetricsChangedEvent {
                                    metrics: metrics.clone(),
                                },
                            );
                            old_metrics = metrics;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    tokio::time::sleep(METRICS_CHECK_INTERVAL).await;
                }
            });
        }
    }
}

#[tauri::command]
fn open_url(app: AppHandle, url: String) -> Result<(), String> {
    let opener = app.opener();
    opener
        .open_url(url.clone(), None::<String>)
        .map_err(|e| format!("Failed to open URL {url}: {e:?}"))?;
    Ok(())
}

#[cfg(desktop)]
#[tauri::command]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn get_protocol_flags(state: State<ManagedState>) -> ProtocolFlags {
    let ipv4_enabled = state.ipv4_enabled.load(Ordering::SeqCst);
    let ipv6_enabled = state.ipv6_enabled.load(Ordering::SeqCst);
    ProtocolFlags {
        ipv4: ipv4_enabled,
        ipv6: ipv6_enabled,
    }
}

fn update_interface(
    current_flag: bool,
    new_flag: bool,
    state_flag: &std::sync::atomic::AtomicBool,
    daemon: &std::sync::MutexGuard<ServiceDaemon>,
    if_kind: &IfKind,
) -> Result<(), String> {
    if current_flag != new_flag {
        if new_flag {
            daemon
                .enable_interface(if_kind.clone())
                .map_err(|e| format!("Failed to enable {if_kind:?} interface: {e:?}"))?;
        } else {
            daemon
                .disable_interface(if_kind.clone())
                .map_err(|e| format!("Failed to disable {if_kind:?} interface: {e:?}"))?;
        }
        state_flag.store(new_flag, Ordering::SeqCst);
    }
    // We have to disable interfaces that are not needed anymore, as enabling IPv4 or IPv6 may also
    // enable those interfaces.
    if let Err(err) = daemon.disable_interface(enumerate_mdns_incapable_interfaces()) {
        // Log the error but continue, as this is not critical.
        log::warn!("Failed to disable interfaces: {err:?}, continuing anyway");
    }
    Ok(())
}

#[tauri::command]
fn set_protocol_flags(state: State<ManagedState>, flags: ProtocolFlags) -> Result<(), String> {
    let daemon = state
        .daemon
        .lock()
        .map_err(|e| format!("Failed to lock daemon: {e:?}"))?;
    let current_ipv4 = state.ipv4_enabled.load(Ordering::SeqCst);
    update_interface(
        current_ipv4,
        flags.ipv4,
        &state.ipv4_enabled,
        &daemon,
        &IfKind::IPv4,
    )?;

    let current_ipv6 = state.ipv6_enabled.load(Ordering::SeqCst);
    update_interface(
        current_ipv6,
        flags.ipv6,
        &state.ipv6_enabled,
        &daemon,
        &IfKind::IPv6,
    )?;

    Ok(())
}

#[cfg(all(target_os = "linux", desktop))]
use webkit2gtk_nvidia_quirk::{set_webkit_disable_dmabuf_renderer, should_disable_dmabuf_renderer};

#[tauri::command]
#[cfg(mobile)]
fn is_desktop() -> bool {
    false
}

#[tauri::command]
#[cfg(desktop)]
fn is_desktop() -> bool {
    true
}

#[tauri::command]
fn copy_to_clipboard(window: Window, contents: String) -> Result<(), String> {
    let app = window.app_handle();
    app.clipboard()
        .write_text(contents)
        .map_err(|e| format!("Failed to copy to clipboard: {e:?}"))?;
    Ok(())
}

#[cfg(desktop)]
#[tauri::command]
fn theme(window: Window) -> Theme {
    match window.theme() {
        Ok(theme) => theme,
        Err(err) => {
            log::error!("Failed to get theme: {err:?}, using dark");
            Theme::Dark
        }
    }
}

#[cfg(desktop)]
#[tauri::command]
fn close_splashscreen(app: AppHandle, state: State<ManagedState>) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.set_focus();
        if state.dev_tools_enabled {
            w.open_devtools();
        }
    }
    if let Some(w) = app.get_webview_window("splashscreen") {
        let _ = w.close();
    }
}

#[cfg(mobile)]
#[tauri::command]
fn theme() -> Theme {
    Theme::Dark
}

#[cfg(desktop)]
#[derive(Parser, Debug)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Args {
    #[arg(
        short = 'l',
        long,
        default_value_t = foreign_crate::LevelFilter::Info,
        value_parser = clap::builder::PossibleValuesParser::new(["trace", "debug", "info", "warn", "error"])
            .map(|s| s.parse::<foreign_crate::LevelFilter>().unwrap_or(foreign_crate::LevelFilter::Info)),
    )]
    log_level: foreign_crate::LevelFilter,
    #[arg(
        short = 'D',
        long,
        default_value_t = false,
        help = "Enable devtools at startup"
    )]
    enable_devtools: bool,
    #[arg(
        short = 'f',
        long,
        default_value_t = false,
        help = "Enable logging to file"
    )]
    log_to_file: bool,
    #[cfg(target_os = "linux")]
    #[arg(
        short = 'd',
        long,
        default_value_t = false,
        help = "Disable dmabuf renderer, useful when having rendering issues"
    )]
    disable_dmabuf_renderer: bool,
}

#[cfg(desktop)]
mod foreign_crate {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub(crate) enum LevelFilter {
        Trace,
        Debug,
        Info,
        Warn,
        Error,
    }

    impl std::fmt::Display for LevelFilter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                Self::Trace => "trace",
                Self::Debug => "debug",
                Self::Info => "info",
                Self::Warn => "warn",
                Self::Error => "error",
            };
            s.fmt(f)
        }
    }
    impl std::str::FromStr for LevelFilter {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "trace" => Ok(Self::Trace),
                "debug" => Ok(Self::Debug),
                "info" => Ok(Self::Info),
                "warn" => Ok(Self::Warn),
                "error" => Ok(Self::Error),
                _ => Err(format!("Unknown log level: {s}")),
            }
        }
    }
    impl From<LevelFilter> for log::LevelFilter {
        fn from(val: LevelFilter) -> Self {
            match val {
                LevelFilter::Trace => log::LevelFilter::Trace,
                LevelFilter::Debug => log::LevelFilter::Debug,
                LevelFilter::Info => log::LevelFilter::Info,
                LevelFilter::Warn => log::LevelFilter::Warn,
                LevelFilter::Error => log::LevelFilter::Error,
            }
        }
    }
}

#[cfg(desktop)]
mod autoupdate {
    use models::UpdateMetadata;
    use serde::Serialize;
    use std::sync::Mutex;
    use tauri::utils::platform::bundle_type;
    use tauri::{AppHandle, State};
    use tauri_plugin_updater::{Update, UpdaterExt};

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Updater(#[from] tauri_plugin_updater::Error),
        #[error("there is no pending update")]
        NoPendingUpdate,
    }

    impl Serialize for Error {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.to_string().as_str())
        }
    }

    type Result<T> = std::result::Result<T, Error>;

    #[tauri::command]
    pub async fn fetch_update(
        app: AppHandle,
        pending_update: State<'_, PendingUpdate>,
    ) -> Result<Option<UpdateMetadata>> {
        let update = app
            .updater_builder()
            .version_comparator(|current, update| update.version != current)
            .build()?
            .check()
            .await?;

        let update_metadata = update.as_ref().map(|update| UpdateMetadata {
            version: update.version.clone(),
            current_version: update.current_version.clone(),
        });

        *pending_update.0.lock().expect("To lock") = update;

        Ok(update_metadata)
    }

    #[tauri::command]
    pub async fn install_update(
        app: AppHandle,
        pending_update: State<'_, PendingUpdate>,
    ) -> Result<()> {
        let Some(update) = pending_update.0.lock().expect("To lock").take() else {
            return Err(Error::NoPendingUpdate);
        };

        let mut downloaded = 0;
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    log::info!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    log::info!("download finished");
                },
            )
            .await?;

        log::info!("update installed, restarting");
        app.restart();
    }

    pub struct PendingUpdate(pub Mutex<Option<Update>>);

    #[tauri::command]
    pub fn can_auto_update() -> bool {
        let current_bundle_type = bundle_type();
        if current_bundle_type.is_none() {
            // Non bundled versions do not support auto updates.
            // We provide plain executables as downloads for users who want to use the app without
            // installation, but those do not have auto-update capabilities.
            // There are also some packaged versions like AUR or XBPS which are handled by the
            // respective package manager, so auto-update is not needed and can be confusing if it
            // is advertised as available.
            log::debug!("Running non-bundled version, auto-update is disabled");
            return false;
        }

        true
    }
}

#[cfg(desktop)]
pub fn run() {
    use chrono::Utc;
    use tauri_plugin_log::{Target, TargetKind};
    let args = Args::parse();

    #[cfg(all(target_os = "linux", desktop))]
    {
        let should_disable = should_disable_dmabuf_renderer(args.disable_dmabuf_renderer);
        if should_disable {
            set_webkit_disable_dmabuf_renderer();
        }
    }

    let mut log_targets = vec![
        Target::new(TargetKind::Stdout),
        Target::new(TargetKind::Webview),
    ];
    if args.log_to_file {
        log_targets.push(Target::new(TargetKind::LogDir { file_name: None }));
    }
    let colors = tauri_plugin_log::fern::colors::ColoredLevelConfig::default();
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(log_targets)
                .level(args.log_level)
                .format(move |out, message, record| {
                    let now = Utc::now();
                    let level = format!("{:<5}", colors.color(record.level()));
                    out.finish(format_args!(
                        "{date} {level} {target}: {message}",
                        date = now.format("%Y-%m-%dT%H:%M:%S%.6fZ"),
                        level = level,
                        target = record.target(),
                        message = message
                    ))
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(ManagedState::new(args.enable_devtools))
        .manage(autoupdate::PendingUpdate(Mutex::new(None)))
        .setup(move |app| {
            // Due to peculiarities of `tauri dev` mode,
            // we need to do close the splashscreen manually
            let main_window = app
                .get_webview_window("main")
                .expect("Main window to exist");
            let url = main_window.url().expect("Main window url to exist");
            let scheme = url.scheme();
            if scheme == "http" {
                if let Some(splashscreen_window) = app.get_webview_window("splashscreen") {
                    tauri::async_runtime::spawn(async move {
                        let _ = splashscreen_window.close();
                        let _ = main_window.show();
                        if args.enable_devtools {
                            main_window.open_devtools();
                        }
                    });
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            autoupdate::fetch_update,
            autoupdate::install_update,
            autoupdate::can_auto_update,
            browse_many,
            browse_types,
            close_splashscreen,
            copy_to_clipboard,
            get_protocol_flags,
            is_desktop,
            open_url,
            set_protocol_flags,
            subscribe_can_browse,
            subscribe_metrics,
            stop_browse,
            theme,
            verify,
            version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(mobile)]
#[tauri::command]
fn close_splashscreen() {}

#[cfg(mobile)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run_mobile() {
    use tauri_plugin_log::{Target, TargetKind};
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(vec![Target::new(TargetKind::Webview)])
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(ManagedState::new())
        .invoke_handler(tauri::generate_handler![
            browse_many,
            browse_types,
            close_splashscreen,
            copy_to_clipboard,
            get_protocol_flags,
            is_desktop,
            open_url,
            set_protocol_flags,
            subscribe_can_browse,
            subscribe_metrics,
            stop_browse,
            theme,
            verify,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
