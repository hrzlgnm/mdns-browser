use std::time::Duration;
pub const MDNS_SD_META_SERVICE: &str = "_services._dns-sd._udp.local.";
pub const METRICS_CHECK_INTERVAL: Duration = Duration::from_secs(1);
pub const INTERFACES_CAN_BROWSE_CHECK_INTERVAL: Duration = Duration::from_millis(500);

#[cfg(debug_assertions)]
pub const SPLASH_SCREEN_DURATION: Duration = Duration::from_secs(0);
#[cfg(not(debug_assertions))]
pub const SPLASH_SCREEN_DURATION: Duration = Duration::from_secs(2);

pub const AUTO_COMPLETE_AUTO_FOCUS_DELAY: Duration = Duration::from_secs(5);
pub const SHOW_NO_UPDATE_DURATION: Duration = Duration::from_secs(3);
pub const GITHUB_BASE_URL: &str = "https://github.com/hrzlgnm/mdns-browser";
pub const VERIFY_TIMEOUT: Duration = Duration::from_secs(5);
