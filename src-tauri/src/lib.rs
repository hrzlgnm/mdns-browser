// Copyright 2024-2026 hrzlgnm
// SPDX-License-Identifier: MIT

#[cfg(desktop)]
use clap::builder::TypedValueParser as _;
#[cfg(desktop)]
use clap::Parser;
#[cfg(desktop)]
use tauri::{AppHandle, Manager};
#[cfg(all(target_os = "linux", desktop))]
use webkit2gtk_nvidia_quirk::{apply_workaround_with_options, ApplyWorkaroundOptions};

#[cfg(mobile)]
use tauri::Manager;

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

#[cfg(desktop)]
#[tauri::command]
fn close_splashscreen(app: AppHandle, dev_tools: tauri::State<'_, bool>) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        if *dev_tools {
            let window = w.clone();
            tauri::async_runtime::spawn(async move {
                #[cfg(target_os = "linux")]
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                window.open_devtools();
            });
        }
    }
    if let Some(w) = app.get_webview_window("splashscreen") {
        let _ = w.close();
    }
}

#[tauri::command]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
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
    #[cfg(target_os = "linux")]
    #[arg(
        short = 'e',
        long,
        default_value_t = false,
        help = "Disable NVIDIA explicit sync even if NVIDIA is not detected"
    )]
    disable_nv_explicit_sync: bool,
    #[cfg(target_os = "linux")]
    #[arg(
        short = 'n',
        long,
        default_value_t = false,
        help = "Disable all NVIDIA workarounds entirely"
    )]
    no_nvidia_workaround: bool,
    #[cfg(target_os = "linux")]
    #[arg(
        short = 'v',
        long,
        default_value_t = false,
        help = "Print diagnostic notes when applying an NVIDIA workaround"
    )]
    nvidia_workaround_verbose: bool,
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
    use tauri::utils::platform::bundle_type;

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
/// Creates the main application window.
///
/// The window is created programmatically (rather than via `tauri.conf.json`)
/// so its decoration state can be decided at creation time, which is the only
/// point at which it reliably takes effect: Wayland/GTK and X11 do not honor
/// runtime decoration changes once the window is mapped. Tiling Wayland
/// compositors therefore start borderless, while every other session starts
/// decorated.
///
/// On non-tiling Wayland the minimize/maximize/close buttons are dead after the
/// window is created hidden and shown (tauri-apps/tao#1046). Starting the
/// window maximized is a creation-time reconfigure that wires the buttons up,
/// avoiding any runtime toggle/cycle.
fn create_main_window(app: &AppHandle) -> Result<tauri::WebviewWindow, tauri::Error> {
    #[cfg(target_os = "linux")]
    let wayland = webkit2gtk_nvidia_quirk::is_wayland_session();
    #[cfg(target_os = "linux")]
    let tiling = webkit2gtk_nvidia_quirk::is_tiling_compositor();
    #[cfg(target_os = "linux")]
    let decorate = !(wayland && tiling);
    #[cfg(target_os = "linux")]
    let start_maximized = wayland && !tiling;
    #[cfg(not(target_os = "linux"))]
    let decorate = true;
    #[cfg(not(target_os = "linux"))]
    let start_maximized = false;

    let mut builder =
        tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
            .title("mDNS-Browser")
            .inner_size(1615.0, 900.0)
            .decorations(decorate)
            .visible(false);
    if start_maximized {
        builder = builder.maximized(true);
    }
    builder.build().map_err(|e| {
        log::error!("Failed to create main window: {e}");
        e
    })
}

#[cfg(desktop)]
#[cfg(test)]
mod args_tests {
    use super::{foreign_crate::LevelFilter, Args};
    use clap::Parser;

    #[test]
    fn test_log_level_parses_known_levels() {
        assert_eq!("trace".parse(), Ok(LevelFilter::Trace));
        assert_eq!("debug".parse(), Ok(LevelFilter::Debug));
        assert_eq!("info".parse(), Ok(LevelFilter::Info));
        assert_eq!("warn".parse(), Ok(LevelFilter::Warn));
        assert_eq!("error".parse(), Ok(LevelFilter::Error));
    }

    #[test]
    fn test_log_level_rejects_unknown_levels() {
        assert!("verbose".parse::<LevelFilter>().is_err());
        assert!("INFO".parse::<LevelFilter>().is_err());
    }

    #[test]
    fn test_args_default_to_info_without_devtools_or_file_logging() {
        let args = Args::try_parse_from(["mdns-browser"]).expect("To parse empty args");
        assert_eq!(args.log_level, LevelFilter::Info);
        assert!(!args.enable_devtools);
        assert!(!args.log_to_file);
    }

    #[test]
    fn test_args_enable_devtools_and_debug_logging() {
        let args = Args::try_parse_from(["mdns-browser", "-D", "-l", "debug"])
            .expect("To parse devtools args");
        assert!(args.enable_devtools);
        assert_eq!(args.log_level, LevelFilter::Debug);
    }
}

#[cfg(desktop)]
pub fn run() {
    use chrono::Utc;
    use tauri_plugin_log::{Target, TargetKind};
    let args = Args::parse();

    #[cfg(target_os = "linux")]
    {
        if !args.no_nvidia_workaround {
            let options = ApplyWorkaroundOptions::default()
                .force_disable_dmabuf(args.disable_dmabuf_renderer)
                .force_disable_nv_explicit_sync(args.disable_nv_explicit_sync)
                .verbose(args.nvidia_workaround_verbose);
            apply_workaround_with_options(options);
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
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_mdns::init())
        .manage(args.enable_devtools)
        .setup(move |app| {
            // The main window is created programmatically (instead of via
            // tauri.conf.json) so its decoration state can be set at creation
            // time. Runtime decoration changes do not take effect on
            // Wayland/GTK (and X11) once the window is mapped, so tiling
            // Wayland compositors must start borderless from the start.
            let main_window = create_main_window(app.handle())?;

            // Due to peculiarities of `tauri dev` mode, we need to close the
            // splashscreen and show the main window manually.
            let url = main_window.url().expect("Main window url to exist");
            let scheme = url.scheme();
            if scheme == "http" {
                if let Some(splashscreen_window) = app.get_webview_window("splashscreen") {
                    tauri::async_runtime::spawn(async move {
                        let _ = splashscreen_window.close();
                        let _ = main_window.show();
                    });
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            autoupdate::can_auto_update,
            close_splashscreen,
            is_desktop,
            version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(mobile)]
#[tauri::command]
fn close_splashscreen() {}

#[cfg(mobile)]
#[tauri::command]
fn can_auto_update() -> bool {
    if cfg!(debug_assertions) {
        log::debug!("Running dev build, auto-update is disabled");
        return false;
    }
    true
}

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
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_android_update::Builder::new()
                .owner("hrzlgnm")
                .repo("mdns-browser")
                .build(),
        )
        .plugin(tauri_plugin_mdns::init())
        .invoke_handler(tauri::generate_handler![
            can_auto_update,
            close_splashscreen,
            is_desktop,
            version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
