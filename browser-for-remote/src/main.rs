#![allow(unused_imports)]

use chrono;
use std::time::Duration;
use headless_chrome::{Browser, Element, LaunchOptions, Tab};
use anyhow::{Result};
use std::thread;
use std::fs;
use std::path::PathBuf;
use serde_json::json;
use std::ffi::OsString;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::{Arc, Mutex};
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;

use std::thread::sleep;
use reqwest::blocking::get;
use serde_json::Value;


const BROWSER_PROFILE_PATH: &str = "/home/profile";
const JSON_OUTPUT_PATH: &str = "/home/rust_chrome_remote/";
const BROWSER_PATH:&str = "/usr/bin/chromium";

fn main() -> Result<()> {
    let mut browser: Option<Arc<Browser>> = None;
    let mut loop_count = 0;

    let dir = env::current_dir()?;
    let profile_path = dir.join("profile/");
    // let json_output_path = dir.clone();

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
    ];

    let mut rng = thread_rng();
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


    loop {
        if browser.is_none() {
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
            let new_browser = Browser::new(launch_options)?;
            browser = Some(Arc::new(new_browser));  // Wrap in Arc.
            println!("Spawned new Chromium instance with profile: {}", BROWSER_PROFILE_PATH);
        }

        thread::sleep(Duration::from_secs(10));

        // Fetch the JSON data from the localhost endpoint.
        let response = get("http://localhost:9222/json")?;
        let data: Value = serde_json::from_str(&response.text()?)?;

        // Count the number of "page" tabs with "chrome://newtab/"
        let newtab_count = data.as_array().unwrap().iter().filter(|tab| {
            tab["type"] == "page" && tab["url"] == "chrome://newtab/"
        }).count();

        if newtab_count < 2 {
            let browser = browser.as_ref().unwrap();  // Access the Arc<Browser>
            let current_tabs = browser.get_tabs().lock().unwrap().clone();
            
            // Open new tabs until there are at least 2 with "chrome://newtab/"
            for _ in 0..(2 - newtab_count) {
                let new_tab = browser.new_tab()?;
                new_tab.navigate_to("chrome://newtab/")?;
            }
            println!("Ensured at least 2 'chrome://newtab/' pages are open.");
        }
    }
}
