// utils.rs

use chrono;

use headless_chrome::{Browser, LaunchOptions};
use std::sync::Arc;
use std::error::Error;
use std::path::PathBuf;
use std::env;
use std::ffi::OsString;
use rand::seq::SliceRandom; // For random selection
use rand::thread_rng; // For random number generator

const BROWSER_PATH:&str = "/usr/bin/chromium";

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
// pub fn create_browser() -> Result<Arc<Browser>, Box<dyn std::error::Error>> {
pub fn create_browser() -> Result<Arc<Browser>, Box<dyn Error>> {

    let dir = env::current_dir()?;
    let profile_path = dir.join("profile/");

    // * LAUNCH BROWSER 
    // For later
    // Replace with your actual IPv6 address
    // let ipv6_address = Ipv6Addr::new(2001, 0db8, 0, 0, 0, 0, 0, 1);
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
    // let disable_gpu = &OsString::from("--disable-gpu");
    let disable_background_timer_throttling = &OsString::from("--disable-background-timer-throttling");
    let disable_backgrounding_occluded_windows = &OsString::from("--disable-backgrounding-occluded-windows");
    let disable_renderer_backgrounding = &OsString::from("--disable-renderer-backgrounding");
    let blink_settings_images_enabled = &OsString::from("--blink-settings=imagesEnabled=false");
    let blink_settings_media_enabled = &OsString::from("--blink-settings=mediaEnabled=false");
    let disable_popup_blocking = &OsString::from("--disable-popup-blocking");
    let disable_features_popups = &OsString::from("--disable-features=Popups");
    let disable_features_css_grid_layout = &OsString::from("--disable-features=CSSGridLayout,CSSGrid");
    let no_experiments = &OsString::from("--no-experiments");
    let disable_infobars = &OsString::from("--disable-infobars");
    // let disable_blink_features_automation_controlled = &OsString::from("--disable-blink-features=AutomationControlled");
    let enable_automation = &OsString::from("--enable-automation");
    // The below only works for headless mode
    let remote_debugging_address = &OsString::from("--remote-debugging-address=0.0.0.0");
    let remote_debugging_port = &OsString::from("--remote-debugging-port=9222");
    let user_agent = &OsString::from("--user-agent=EXODUS");

    let mut launch_options = LaunchOptions::default();
    launch_options.headless = false;
    launch_options.user_data_dir = Some(PathBuf::from(profile_path.clone()));
    launch_options.window_size = window_size;
    launch_options.path = Some(PathBuf::from(BROWSER_PATH));  // Use BROWSER_PATH here.

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
        // disable_gpu,
        disable_background_timer_throttling,
        disable_backgrounding_occluded_windows,
        disable_renderer_backgrounding,
        blink_settings_images_enabled,
        blink_settings_media_enabled,
        disable_popup_blocking,
        disable_features_popups,
        disable_features_css_grid_layout,
        no_experiments,
        disable_infobars,
        // disable_blink_features_automation_controlled,
        enable_automation,
        remote_debugging_address,
        remote_debugging_port,
        user_agent,
    ];
    let browser = Browser::new(launch_options)?;
    Ok(Arc::new(browser.into()))
}