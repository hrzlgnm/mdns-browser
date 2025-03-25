#[cfg(desktop)]
use clap::builder::TypedValueParser as _;
#[cfg(desktop)]
use clap::Parser;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use models::check_service_type_fully_qualified;
use models::*;
#[cfg(all(desktop, not(debug_assertions)))]
use shared_constants::SPLASH_SCREEN_DURATION;
use shared_constants::{MDNS_SD_META_SERVICE, METRICS_CHECK_INTERVAL, VERIFY_TIMEOUT};
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, Mutex},
};
use tauri::Emitter;
use tauri::{AppHandle, Manager, State, Window};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_opener::OpenerExt;

#[cfg(desktop)]
use tauri_plugin_log::{Target, TargetKind};

type SharedServiceDaemon = Arc<Mutex<ServiceDaemon>>;

struct ManagedState {
    daemon: SharedServiceDaemon,
    running_browsers: Arc<Mutex<Vec<String>>>,
}

impl ManagedState {
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

fn from_service_info(info: &ServiceInfo) -> ResolvedService {
    let mut sorted_addresses: Vec<IpAddr> = info.get_addresses().clone().drain().collect();
    sorted_addresses.sort();
    let mut sorted_txt: Vec<TxtRecord> = info
        .get_properties()
        .iter()
        .map(|r| TxtRecord {
            key: r.key().into(),
            val: bytes_option_to_string_option_with_escaping(r.val()),
        })
        .collect();
    sorted_txt.sort_by(|a, b| a.key.partial_cmp(&b.key).expect("To be partial comparable"));
    ResolvedService {
        instance_name: info.get_fullname().into(),
        service_type: info.get_type().into(),
        hostname: info.get_hostname().into(),
        port: info.get_port(),
        addresses: sorted_addresses,
        subtype: info.get_subtype().clone(),
        txt: sorted_txt,
        updated_at_ms: timestamp_millis(),
        dead: false,
    }
}

fn emit_event<T>(window: &Window, event: &str, payload: &T)
where
    T: serde::Serialize,
{
    if let Err(e) = window.emit(event, payload) {
        log::error!("Failed to emit {} event: {:?}", event, e);
    }
}

#[tauri::command]
fn browse_types(window: Window, state: State<ManagedState>) {
    if let Ok(mdns) = state.daemon.lock() {
        let mdns_for_thread = mdns.clone();
        std::thread::spawn(move || {
            let receiver = match mdns_for_thread.browse(MDNS_SD_META_SERVICE) {
                Ok(receiver) => receiver,
                Err(e) => {
                    log::error!("Failed to browse for service types: {:?}", e);
                    return;
                }
            };
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceFound(_service_type, full_name) => {
                        log::debug!("Service type found: {}", full_name);
                        match check_service_type_fully_qualified(full_name.as_str()) {
                            Ok(_) => {
                                emit_event(
                                    &window,
                                    "service-type-found",
                                    &ServiceTypeFoundEvent {
                                        service_type: full_name,
                                    },
                                );
                            }
                            Err(e) => {
                                log::warn!("Ignoring invalid service type `{full_name}`: {}", e)
                            }
                        }
                    }
                    ServiceEvent::ServiceRemoved(_service_type, full_name) => {
                        log::debug!("Service type removed: {}", full_name);
                        match check_service_type_fully_qualified(full_name.as_str()) {
                            Ok(_) => {
                                emit_event(
                                    &window,
                                    "service-type-removed",
                                    &ServiceTypeRemovedEvent {
                                        service_type: full_name.clone(),
                                    },
                                );
                            }
                            Err(e) => {
                                log::warn!("Ignoring invalid service type `{full_name}`: {}", e);
                            }
                        }
                    }
                    ServiceEvent::SearchStopped(service_type) => {
                        if service_type == MDNS_SD_META_SERVICE {
                            break;
                        }
                    }
                    _ => {
                        log::debug!("Ignoring event: {:?}", event);
                    }
                }
            }
            log::debug!("Browse type thread ending.");
        });
    }
}

#[tauri::command]
fn stop_browse(state: State<ManagedState>) {
    if let Ok(mdns) = state.daemon.lock() {
        if let Ok(mut running_browsers) = state.running_browsers.lock() {
            running_browsers.iter().for_each(|ty_domain| {
                if let Err(e) = mdns.stop_browse(ty_domain) {
                    log::error!("Failed to stop browsing for {ty_domain}: {:?}", e);
                }
            });
            running_browsers.clear();
        }
    }
}

#[tauri::command]
fn verify(instance_fullname: String, state: State<ManagedState>) {
    log::debug!("verifying {}", instance_fullname);
    if let Ok(mdns) = state.daemon.lock() {
        if let Err(e) = mdns.verify(instance_fullname.clone(), VERIFY_TIMEOUT) {
            log::error!("Failed to verify {instance_fullname}: {:?}", e);
        }
    }
}

#[tauri::command]
fn browse_many(service_types: Vec<String>, window: Window, state: State<ManagedState>) {
    for service_type in service_types {
        if let Ok(mdns) = state.daemon.lock() {
            if let Ok(mut running_browsers) = state.running_browsers.lock() {
                if !running_browsers.contains(&service_type) {
                    running_browsers.push(service_type.clone());
                    let receiver = match mdns.browse(service_type.as_str()) {
                        Ok(receiver) => receiver,
                        Err(e) => {
                            log::error!(
                                "Failed to start browsing for {service_type} browse: {:?}",
                                e,
                            );
                            return;
                        }
                    };
                    let window = window.clone();
                    std::thread::spawn(move || {
                        while let Ok(event) = receiver.recv() {
                            match event {
                                ServiceEvent::ServiceFound(_service_type, instance_name) => {
                                    emit_event(
                                        &window,
                                        "service-found",
                                        &ServiceFoundEvent {
                                            instance_name,
                                            at_ms: timestamp_millis(),
                                        },
                                    )
                                }
                                ServiceEvent::SearchStarted(service_type) => emit_event(
                                    &window,
                                    "search-started",
                                    &SearchStartedEvent { service_type },
                                ),
                                ServiceEvent::ServiceResolved(info) => emit_event(
                                    &window,
                                    "service-resolved",
                                    &ServiceResolvedEvent {
                                        service: from_service_info(&info),
                                    },
                                ),

                                ServiceEvent::ServiceRemoved(_service_type, instance_name) => {
                                    emit_event(
                                        &window,
                                        "service-removed",
                                        &ServiceRemovedEvent {
                                            instance_name,
                                            at_ms: timestamp_millis(),
                                        },
                                    );
                                }
                                ServiceEvent::SearchStopped(service_type) => {
                                    emit_event(
                                        &window,
                                        "search-stopped",
                                        &SearchStoppedEvent { service_type },
                                    );
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
}

#[tauri::command]
fn subscribe_metrics(window: Window, state: State<ManagedState>) {
    if let Ok(mdns) = state.daemon.lock() {
        let mdns_for_thread = mdns.clone();
        let mut old_metrics = HashMap::new();
        std::thread::spawn(move || loop {
            std::thread::sleep(METRICS_CHECK_INTERVAL);
            if let Ok(metrics_receiver) = mdns_for_thread.get_metrics() {
                if let Ok(metrics) = metrics_receiver.recv() {
                    if old_metrics != metrics {
                        emit_event(
                            &window,
                            "metrics",
                            &MetricsEvent {
                                metrics: metrics.clone(),
                            },
                        );
                        old_metrics = metrics;
                    }
                }
            }
        });
    }
}

#[tauri::command]
fn open_url(app: AppHandle, url: String) {
    let opener = app.opener();
    let r = opener.open_url(url.clone(), None::<String>);
    if r.is_err() {
        log::error!("Failed to open {}: {:?}", url, r);
    }
}

#[cfg(desktop)]
#[tauri::command]
fn version(window: Window) -> String {
    window
        .app_handle()
        .config()
        .version
        .clone()
        .unwrap_or(String::from("Unknown"))
}

#[cfg(desktop)]
#[cfg(target_os = "linux")]
fn x11_workaround() {
    let session_type_key = "XDG_SESSION_TYPE";
    match std::env::var(session_type_key) {
        Ok(val) => {
            if val == "x11" {
                println!(
                    "Setting WEBKIT_DISABLE_DMABUF_RENDERER=1 to workaround rendering issues with x11 session"
                );
                std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1")
            }
        }
        Err(_e) => {}
    }
}

#[cfg(desktop)]
#[derive(Parser, Debug)]
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
        short = 'f',
        long,
        default_value_t = false,
        help = "Enable logging to file"
    )]
    log_to_file: bool,
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
#[tauri::command]
fn is_desktop() -> bool {
    true
}

#[cfg(desktop)]
#[tauri::command]
fn can_auto_update() -> bool {
    #[cfg(target_os = "linux")]
    {
        false
    }
    #[cfg(not(target_os = "linux"))]
    {
        true
    }
}

#[cfg(mobile)]
#[tauri::command]
fn is_desktop() -> bool {
    false
}

#[tauri::command]
fn copy_to_clipboard(window: Window, contents: String) {
    let app = window.app_handle();
    if let Err(e) = app.clipboard().write_text(contents) {
        log::error!("Failed to copy to clipboard: {}", e);
    }
}

#[cfg(desktop)]
mod app_updates {
    use models::UpdateMetadata;
    use serde::Serialize;
    use std::sync::Mutex;
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
}

#[cfg(desktop)]
pub fn run() {
    #[cfg(target_os = "linux")]
    x11_workaround();
    let args = Args::parse();
    let mut log_targets = vec![
        Target::new(TargetKind::Stdout),
        Target::new(TargetKind::Webview),
    ];
    if args.log_to_file {
        log_targets.push(Target::new(TargetKind::LogDir { file_name: None }));
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(ManagedState::new())
        .manage(app_updates::PendingUpdate(Mutex::new(None)))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(log_targets)
                .level(args.log_level)
                .build(),
        )
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let splashscreen_window = app
                .get_webview_window("splashscreen")
                .expect("Splashscreen window to exist");
            let main_window = app
                .get_webview_window("main")
                .expect("Main window to exist");
            tauri::async_runtime::spawn(async move {
                #[cfg(not(debug_assertions))]
                tokio::time::sleep(SPLASH_SCREEN_DURATION).await;
                splashscreen_window.close().expect("To close");
                main_window.show().expect("To show");
                #[cfg(debug_assertions)]
                main_window.open_devtools();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_updates::fetch_update,
            app_updates::install_update,
            browse_many,
            browse_types,
            can_auto_update,
            copy_to_clipboard,
            is_desktop,
            open_url,
            subscribe_metrics,
            stop_browse,
            verify,
            version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(mobile)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run_mobile() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(ManagedState::new())
        .invoke_handler(tauri::generate_handler![
            browse_many,
            browse_types,
            copy_to_clipboard,
            is_desktop,
            open_url,
            subscribe_metrics,
            stop_browse,
            verify,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
