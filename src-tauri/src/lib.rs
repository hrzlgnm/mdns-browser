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
use std::sync::atomic::{AtomicBool, Ordering};
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
    metrics_subscribed: AtomicBool,
}

impl ManagedState {
    fn new() -> Self {
        Self {
            daemon: get_shared_daemon(),
            running_browsers: Arc::new(Mutex::new(Vec::new())),
            metrics_subscribed: AtomicBool::new(false),
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

/// Emits an event to the window.
///
/// This helper centralizes event emission and handles any errors internally.
/// If window.emit returns an error, it is logged and not propagated.
/// Use this helper to avoid repetitive error handling.
fn emit_event<T>(window: &Window, event: &str, payload: &T)
where
    T: serde::Serialize,
{
    if let Err(e) = window.emit(event, payload) {
        log::error!("Failed to emit {} event: {:?}", event, e);
    }
}

#[tauri::command]
/// Starts browsing for mDNS service types and emits events on the specified window when services are found or removed.
///
/// The function stops any previous browsing for the meta service type before launching an asynchronous task that listens
/// for mDNS events. For each event received, it validates the service type; if the service's full name is valid, it emits
/// either a "service-type-found" or "service-type-removed" event via the provided window. The browsing task terminates once
/// a stop signal for the meta service type is received.
///
/// # Examples
///
/// ```rust
/// // Example usage within a Tauri application context.
/// // Assume `window` is a valid Tauri Window and `state` is the ManagedState containing an mDNS daemon.
/// browse_types(window, state);
/// ```
fn browse_types(window: Window, state: State<ManagedState>) {
    if let Ok(mdns) = state.daemon.lock() {
        let mdns_for_task = mdns.clone();
        if mdns_for_task.stop_browse(MDNS_SD_META_SERVICE).is_ok() {
            log::trace!("Stopped browsing for service types");
        }
        tauri::async_runtime::spawn(async move {
            let receiver = match mdns_for_task.browse(MDNS_SD_META_SERVICE) {
                Ok(receiver) => receiver,
                Err(e) => {
                    log::error!("Failed to browse for service types: {:?}", e);
                    return;
                }
            };
            while let Ok(event) = receiver.recv_async().await {
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
                                log::warn!("Ignoring invalid service type `{}`: {}", full_name, e)
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
                                log::warn!("Ignoring invalid service type `{}`: {}", full_name, e)
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
            log::debug!("Browse type task ending.");
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
                    tauri::async_runtime::spawn(async move {
                        while let Ok(event) = receiver.recv_async().await {
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
                        log::debug!("Browse task for {} ending.", &service_type);
                    });
                }
            }
        }
    }
}

#[tauri::command]
fn subscribe_metrics(window: Window, state: State<ManagedState>) {
    // Avoid multiple subscriptions when the frontend is reloaded.
    if state
        .metrics_subscribed
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        if let Ok(mdns) = state.daemon.lock() {
            let mdns_for_task = mdns.clone();
            let mut old_metrics = HashMap::new();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(METRICS_CHECK_INTERVAL).await;
                    if let Ok(metrics_receiver) = mdns_for_task.get_metrics() {
                        if let Ok(metrics) = metrics_receiver.recv_async().await {
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
                }
            });
        }
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
mod linux {
    use std::process::Command;
    /// Checks for NVIDIA usage via the system's `glxinfo` command.
    ///
    /// This function first verifies that `glxinfo` is installed. If not, it prints a warning to standard error
    /// and returns an error. When installed, it executes `glxinfo` to extract the "OpenGL renderer string"
    /// and checks (in a case-insensitive manner) whether it contains the substring "nvidia". It returns:
    /// - `Ok(true)` if an NVIDIA-based renderer is detected.
    /// - `Ok(false)` if no NVIDIA usage is identified or the command's output doesn't indicate NVIDIA.
    /// - `Err(())` if `glxinfo` is not installed.
    /// 
    /// # Examples
    ///
    /// ```
    /// // Note: Ensure that `glxinfo` is installed for this example to work as expected.
    /// match check_nvidia_glxinfo() {
    ///     Ok(true) => println!("NVIDIA graphics are in use."),
    ///     Ok(false) => println!("NVIDIA graphics are not in use."),
    ///     Err(()) => println!("Warning: glxinfo is not installed."),
    /// }
    /// ```
    fn check_nvidia_glxinfo() -> Result<bool, ()> {
        let is_glxinfo_installed = Command::new("sh")
            .arg("-c")
            .arg("command -v glxinfo")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);

        if !is_glxinfo_installed {
            eprintln!("Warning: glxinfo is not installed, cannot detect whether NVIDIA is used.");
            return Err(());
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg("glxinfo | grep 'OpenGL renderer string'")
            .output();

        if let Ok(out) = output {
            let out_str = String::from_utf8_lossy(&out.stdout);
            return Ok(out_str.to_lowercase().contains("nvidia"));
        }

        Ok(false)
    }

    /// Determines whether dmabuf rendering should be disabled based on system conditions and a force flag.
    /// 
    /// When `force_disable` is true, the function immediately returns `Ok(true)` and prints a notice
    /// about degraded renderer performance. Otherwise, it checks for basic platform conditions—verifying
    /// that `/dev/dri` exists, ensuring that the session is not running under Wayland, and confirming an X11 session.
    /// If those conditions are not met, it returns `Ok(false)`. When the conditions indicate an X11 environment,
    /// the function calls `check_nvidia_glxinfo` to detect an NVIDIA GPU; if one is found, it prints additional
    /// warnings and returns `Ok(true)`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // Stub implementation for demonstration purposes.
    /// fn check_nvidia_glxinfo() -> Result<bool, ()> { Ok(false) }
    /// 
    /// // Example: System conditions determine whether to disable dmabuf rendering.
    /// let disable = should_disable_dmabuf(false).unwrap();
    /// if disable {
    ///     // Adjust configuration for degraded renderer performance.
    /// }
    /// 
    /// // Example: Forcefully disable dmabuf rendering.
    /// let force_disable = should_disable_dmabuf(true).unwrap();
    /// assert!(force_disable);
    /// ```
    fn should_disable_dmabuf(force_disable: bool) -> Result<bool, ()> {
        // Return true immediately if forced
        if force_disable {
            eprintln!("Note: dmabuf renderer disabled by command line arg. Expect degraded renderer performance");
            return Ok(true);
        }
        // Check basic platform conditions
        if !std::path::Path::new("/dev/dri").exists()
            || std::env::var("WAYLAND_DISPLAY").is_ok()
            || std::env::var("XDG_SESSION_TYPE").unwrap_or_default() != "x11"
        {
            return Ok(false);
        }
        // Check for Nvidia via glxinfo
        let nvidia_detected = check_nvidia_glxinfo()?;
        if nvidia_detected {
            eprintln!("Note: nvidia with XOrg detected, disabling dmabuf renderer. Expect degraded renderer performance.");
            eprintln!("See https://github.com/hrzlgnm/mdns-browser/issues/947 for more details.");
        }
        Ok(nvidia_detected)
    }

    /// Disables WebKit's DMABUF renderer when system conditions or a forced override require it.
    /// 
    /// This function determines whether to disable DMABUF rendering by calling
    /// `should_disable_dmabuf` with the provided `force_disable` flag. If the check returns
    /// `Ok(true)`, the environment variable `WEBKIT_DISABLE_DMABUF_RENDERER` is set to `"1"`,
    /// effectively disabling the DMABUF renderer in WebKit.
    /// 
    /// **Note:** The environment variable is set within an unsafe block, which may lead to
    /// potential race conditions in multi-threaded contexts.
    /// 
    /// # Parameters
    /// 
    /// - `force_disable`: When true, forces the disabling of DMABUF rendering regardless of
    ///   the standard detection logic.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use std::env;
    /// 
    /// // Ensure the environment variable is not set.
    /// env::remove_var("WEBKIT_DISABLE_DMABUF_RENDERER");
    /// 
    /// // Invoke the function with a forced disable.
    /// disable_webkit_dmabuf_rendering_if_needed(true);
    /// 
    /// // Confirm that the environment variable has been set correctly.
    /// assert_eq!(env::var("WEBKIT_DISABLE_DMABUF_RENDERER").unwrap(), "1");
    /// ```
    pub fn disable_webkit_dmabuf_rendering_if_needed(force_disable: bool) {
        if let Ok(disable) = should_disable_dmabuf(force_disable) {
            if disable {
                // SAFETY: There's potential for race conditions in a multi-threaded context.
                unsafe {
                    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
                }
            }
        }
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
/// Initializes and launches the Tauri application with configured plugins, logging, and window management.
///
/// This function parses command-line arguments to configure logging targets (including optional file logging)
/// and conditionally disables dmabuf rendering on Linux systems. It sets up the Tauri builder with plugins for
/// clipboard management, logging, auto-updates, and more, and manages application state. During setup, it handles
/// the transition from a splash screen to the main window, spawning an asynchronous task to close the splash screen
/// and display the main window (with developer tools enabled in debug mode). The application is then run with a
/// generated context, and the function will panic if the startup process fails.
///
/// # Examples
///
/// ```rust
/// #[test]
/// fn test_run_starts_without_immediate_panic() {
///     // This test verifies that the run function can be invoked without immediately panicking.
///     // Note: The full application lifecycle is not executed in this test environment.
///     let result = std::panic::catch_unwind(|| {
///         run();
///     });
///     assert!(result.is_ok());
/// }
/// ```
pub fn run() {
    let args = Args::parse();

    #[cfg(target_os = "linux")]
    linux::disable_webkit_dmabuf_rendering_if_needed(args.disable_dmabuf_renderer);

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
