// utils.rs

use chrono;

use headless_chrome::{Browser, LaunchOptions};
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

// const BROWSER_HEADLESS: bool = false;
const BROWSER_HEADLESS: bool = true;
const BROWSER_PATH: &str = "/usr/bin/chromium-browser";

/// Logs a message with the current date and time.
///
/// # Arguments
///
/// * `message` - A string slice that holds the message to be logged.
///
/// # Example
///
/// ```
/// log_message("Removing cookie consent pop-up.");
/// ```
pub fn log_message(message: &str, level: &str) {
    println!(
        "[{}][{}] {}",
        level,
        chrono::Local::now().format("%d/%m/%y %H:%M:%S"),
        message
    );
}
pub fn create_browser(browser_profile_path: &str) -> Result<Arc<Browser>, Box<dyn Error>> {
    // * LAUNCH BROWSER
    // For later
    // Replace with your actual IPv6 address
    // let ipv6_address = Ipv6Addr::new(2001, 0db8, 0, 0, 0, 0, 0, 1);

    // Common paths to check for Chrome/Chromium executables
    let browser_paths = vec![
        "/usr/bin/chrome",
        "/usr/bin/chromium",
        "/usr/bin/google-chrome",
        "/usr/bin/chromium-browser",
        "/opt/google/chrome/chrome",
        "/snap/bin/chromium",
        // Add any additional paths you may want to check
    ];

    // Try to find the browser executable from the list of possible paths
    let browser_path = browser_paths
        .iter()
        .find(|&&path| PathBuf::from(path).exists())
        .map(|&path| path)
        .unwrap_or(BROWSER_PATH); // Fallback to default if no paths are found

    // Create a new headless Chrome browser instance
    let resolutions = [
        [1440, 1080],
        [1080, 1200],
        [1280, 1024],
        [1920, 1400],
        [1920, 1440],
        [1440, 1080],
        [1440, 1024],
        [1400, 1050],
        [1600, 1024],
        [1600, 1050],
        [1680, 1050],
        [1200, 1040],
        [1600, 1200],
        [1600, 1280],
        [1920, 1080],
        [1920, 1200],
        [1920, 1080],
        // [2560, 1080],
        // [2560, 1440],
    ];
    // Create a random number generator
    let mut rng = thread_rng();
    // Select a random element from the array
    let random_resolution = resolutions.choose(&mut rng);
    let window_size: Option<(u32, u32)> = match random_resolution {
        Some(&[width, height]) => Some((width as u32, height as u32)),
        None => Some((1200, 1040)), // Default resolution
    };

    // Create &OsString values with a longer lifetime.
    let no_sandbox = &OsString::from("--no-sandbox");
    let disable_translate = &OsString::from("--disable-translate");
    let disable_default_apps = &OsString::from("--disable-default-apps");
    let disable_accelerated_2d_canvas = &OsString::from("--disable-accelerated-2d-canvas");
    let no_first_run = &OsString::from("--no-first-run");
    let disable_geolocation = &OsString::from("--disable-geolocation");
    // let disable_webrtc = &OsString::from("--disable-webrtc");
    let disable_background_sync = &OsString::from("--disable-background-sync");
    let disable_service_workers = &OsString::from("--disable-service-workers");
    // let disable_dev_tools = &OsString::from("--disable-dev-tools");
    let disk_cache_size = &OsString::from("--disk-cache-size=104857600"); // 100MB disk cache size
    let media_cache_size = &OsString::from("--media-cache-size=104857600"); // 100MB media cache size
    let app_cache_force_enabled = &OsString::from("--app-cache-force-enabled");
    let disable_gpu = &OsString::from("--disable-gpu");
    let disable_background_timer_throttling =
        &OsString::from("--disable-background-timer-throttling");
    let disable_backgrounding_occluded_windows =
        &OsString::from("--disable-backgrounding-occluded-windows");
    let disable_renderer_backgrounding = &OsString::from("--disable-renderer-backgrounding");
    let blink_settings_images_enabled = &OsString::from("--blink-settings=imagesEnabled=false");
    let blink_settings_media_enabled = &OsString::from("--blink-settings=mediaEnabled=false");
    let disable_popup_blocking = &OsString::from("--disable-popup-blocking");
    // let disable_features_popups = &OsString::from("--disable-features=Popups");
    let disable_features = &OsString::from(
        "--disable-features=CSSGridLayout,CSSGrid,CalculateNativeWinOcclusion,Popups",
    );
    let no_experiments = &OsString::from("--no-experiments");
    let disable_infobars = &OsString::from("--disable-infobars");
    // let disable_blink_features_automation_controlled = &OsString::from("--disable-blink-features=AutomationControlled");
    let enable_automation = &OsString::from("--enable-automation");
    // The below only works for headless mode
    let remote_debugging_address = &OsString::from("--remote-debugging-address=0.0.0.0");
    let remote_debugging_port = &OsString::from("--remote-debugging-port=9222");
    // Setting the below will stop the tabs from setting there own.
    // let user_agent = &OsString::from("--user-agent=EXODUS");

    let disable_backgrounding_ramme_processes =
        &OsString::from("--disable-backgrounding-ramme-processes");
    let disable_ipc_flooding_protection = &OsString::from("--disable-ipc-flooding-protection");
    let disable_extensions = &OsString::from("--disable-extensions");
    let disable_sync = &OsString::from("--disable-sync");
    let disable_logging = &OsString::from("--disable-logging");
    let mut launch_options = LaunchOptions::default();
    launch_options.headless = BROWSER_HEADLESS;
    launch_options.user_data_dir = Some(PathBuf::from(browser_profile_path));
    launch_options.window_size = window_size;

    launch_options.path = Some(PathBuf::from(browser_path)); // Use found browser path

    launch_options.args = vec![
        no_sandbox,
        disable_translate,
        disable_default_apps,
        disable_accelerated_2d_canvas,
        no_first_run,
        disable_geolocation,
        // disable_webrtc,
        disable_background_sync,
        disable_service_workers,
        // disable_dev_tools,
        disk_cache_size,
        media_cache_size,
        app_cache_force_enabled,
        disable_gpu,
        disable_background_timer_throttling,
        disable_backgrounding_occluded_windows,
        disable_renderer_backgrounding,
        blink_settings_images_enabled,
        blink_settings_media_enabled,
        disable_popup_blocking,
        // disable_features_popups,
        disable_features,
        no_experiments,
        disable_infobars,
        // disable_blink_features_automation_controlled,
        enable_automation,
        remote_debugging_address,
        remote_debugging_port,
        // user_agent,
        disable_backgrounding_ramme_processes,
        disable_ipc_flooding_protection,
        disable_extensions,
        disable_sync,
        disable_logging,
    ];
    let browser = Browser::new(launch_options)?;
    Ok(Arc::new(browser.into()))
}
/// Returns the path to the profile directory for browser use.
///
/// If the `HOME` environment variable is set, it will return the path
/// `~/.browser-for-remote/`. If `HOME` is not set, it will generate a
/// fallback path in `/tmp/browser-for-remote/` with a random suffix.
///
/// The function ensures that the directory exists by creating it if
/// it does not already exist.
///
/// # Panics
/// This function will panic if the directory cannot be created.
pub fn get_profile_path() -> String {
    let profile_path = match env::var("HOME") {
        Ok(home_dir) => format!("{}/.browser-for-remote/", home_dir),
        Err(_) => {
            // Generate a random string to append to /tmp/browser-for-remote/
            let random_string: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();
            format!("/tmp/browser-for-remote/{}", random_string)
        }
    };

    // Create the directory if it doesn't exist
    fs::create_dir_all(&profile_path).expect("Failed to create profile directory");

    profile_path
}
