mod utils;

use anyhow::Result;
use headless_chrome::{Browser, Tab};
use reqwest::blocking::get;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use utils::{create_browser, get_profile_path, log_message};

// Struct to hold tab metadata
struct TabMetadata {
    open_time: Instant,
    last_url_change_time: Instant,
    current_url: String,
    tab: Arc<Tab>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut browser: Option<Arc<Browser>> = None;
    let browser_profile_path = get_profile_path();
    // Track tabs and their open times
    let tab_metadata: Arc<Mutex<HashMap<String, TabMetadata>>> =
        Arc::new(Mutex::new(HashMap::new()));

    loop {
        if browser.is_none() {
            match create_browser(&browser_profile_path) {
                Ok(b) => {
                    browser = Some(b);
                    log_message(
                        &format!(
                            "Spawned new Chromium instance with profile: {}",
                            browser_profile_path
                        ),
                        "INFO",
                    );
                }
                Err(e) => {
                    log_message(
                        &format!("Failed to create browser instance: {}", e),
                        "ERROR",
                    );
                    // Optional: Add a delay or retry mechanism if desired
                    std::thread::sleep(std::time::Duration::from_secs(5)); // Delay for retry
                }
            }
        }

        // Fetch the JSON data from the localhost endpoint.
        let response = get("http://localhost:9222/json")?;
        let data: Value = serde_json::from_str(&response.text()?)?;

        let external_tabs: Vec<&Value> = data
            .as_array()
            .unwrap()
            .iter()
            .filter(|tab| {
                tab["type"] == "page"
                    && (tab["url"] != "chrome://newtab/" && tab["url"] != "about:blank")
            })
            .collect();

        // Lock the tab_metadata to safely update it
        let mut tab_metadata_lock = tab_metadata.lock().unwrap();

        // Iterate over tab metadata and close those open for more than 5 minutes or already closed
        tab_metadata_lock.retain(|tab_id, metadata| {
            // Check if the tab is still in the current list of open tabs
            let tab_still_open = external_tabs
                .iter()
                .any(|tab| tab["id"].as_str() == Some(tab_id));

            if !tab_still_open {
                log_message(&format!("Tab already closed: {}", tab_id), "INFO");
                return false; // Remove from metadata since it's already closed
            }

            // Get the current URL of the tab
            if let Some(tab_data) = external_tabs
                .iter()
                .find(|tab| tab["id"].as_str() == Some(tab_id))
            {
                if let Some(current_url) = tab_data["url"].as_str() {
                    // If the URL has changed, update the metadata
                    if current_url != metadata.current_url {
                        metadata.current_url = current_url.to_string();
                        metadata.last_url_change_time = Instant::now();
                    }
                }
            }

            // Check if the tab has been on the same URL for more than 5 minutes
            if metadata.last_url_change_time.elapsed() > Duration::from_secs(300) {
                log_message(
                    &format!("Closing tab (on same URL for over 5 minutes): {}", tab_id),
                    "INFO",
                );
                if let Err(e) = metadata.tab.close_with_unload() {
                    log_message(&format!("Failed to close tab: {}", e), "ERROR");
                }
                return false; // Remove closed tab from metadata
            }

            true // Keep tab in metadata if it's still open and URL hasn't stayed unchanged for 5 minutes
        });

        for tab in &external_tabs {
            if let Some(tab_id) = tab["id"].as_str() {
                if !tab_metadata_lock.contains_key(tab_id) {
                    // If it's a new tab, add it to the metadata
                    let browser_tabs = browser
                        .as_ref()
                        .expect("Browser instance not available")
                        .get_tabs()
                        .lock()
                        .unwrap();
                    if let Some(new_tab) = browser_tabs
                        .iter()
                        .find(|tab| tab.get_target_id() == tab_id)
                    {
                        if let Some(current_url) = tab["url"].as_str() {
                            tab_metadata_lock.insert(
                                tab_id.to_string(),
                                TabMetadata {
                                    open_time: Instant::now(),
                                    last_url_change_time: Instant::now(),
                                    current_url: current_url.to_string(),
                                    tab: Arc::clone(new_tab),
                                },
                            );
                        }
                    }
                }
            }
        }

        for tab in &external_tabs {
            if let Some(url) = tab["url"].as_str() {
                log_message(&format!("URL: {}", url), "INFO");
            }
        }
        // Count the number of "page" tabs with "chrome://newtab/"
        let newtab_count = data
            .as_array()
            .unwrap()
            .iter()
            .filter(|tab| {
                tab["type"] == "page"
                    && (tab["url"] == "chrome://newtab/" || tab["url"] == "about:blank")
            })
            .count();
        if newtab_count < 1 {
            let browser = browser.as_ref().unwrap(); // Access the Arc<Browser>

            // Try to open a new tab
            let new_tab = match browser.new_tab() {
                Ok(tab) => tab,
                Err(e) => {
                    log_message(&format!("Failed to open new tab: {}", e), "ERROR");
                    // return Err(Box::new(e)); // Handle error as per your logic
                    return Ok(()); // Log the error and continue without crashing
                }
            };

            // Attempt to navigate to "chrome://newtab/", and if it fails, navigate to "about:blank"
            if let Err(e) = new_tab.navigate_to("chrome://newtab/") {
                log_message(&format!("Failed to navigate to 'chrome://newtab/': {}. Navigating to 'about:blank' instead.", e), "ERROR");

                // Attempt to navigate to 'about:blank' as a fallback
                if let Err(e) = new_tab.navigate_to("about:blank") {
                    log_message(
                        &format!("Failed to navigate to 'about:blank': {}", e),
                        "ERROR",
                    );
                    return Ok(()); // Log the error and continue without crashing
                }
            }

            // // Open new tab with "chrome://newtab/"
            // let new_tab = browser.new_tab()?;
            // new_tab.navigate_to("chrome://newtab/")?;
            // log_message(
            //     "Ensured at least 1 'chrome://newtab/' pages are open.",
            //     "INFO",
            // );
        }
        thread::sleep(Duration::from_secs(10));
    }
}
