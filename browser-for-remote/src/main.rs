mod utils;

use utils::{log_message, create_browser};
use std::time::Duration;
use headless_chrome::Browser;
use anyhow::Result;
use std::thread;
use std::sync::Arc;
use reqwest::blocking::get;
use serde_json::Value;
use std::error::Error;

const BROWSER_PROFILE_PATH: &str = "/home/profile";
// const JSON_OUTPUT_PATH: &str = "/home/rust_chrome_remote/";

fn main() -> Result<(), Box<dyn Error>> {
    let mut browser: Option<Arc<Browser>> = None;
    // let mut loop_count = 0;
    // let dir = env::current_dir()?;
    // let json_output_path = dir.clone();

    loop {
        if browser.is_none() {
            match create_browser() {
                Ok(b) => {
                    browser = Some(b);
                    log_message(&format!("Spawned new Chromium instance with profile: {}", BROWSER_PROFILE_PATH), "INFO");
                },
                Err(e) => {
                    log_message(&format!("Failed to create browser instance: {}", e), "ERROR");
                    // Optional: Add a delay or retry mechanism if desired
                    std::thread::sleep(std::time::Duration::from_secs(5)); // Delay for retry
                }
            }
        }

        thread::sleep(Duration::from_secs(10));

        // Fetch the JSON data from the localhost endpoint.
        let response = get("http://localhost:9222/json")?;
        let data: Value = serde_json::from_str(&response.text()?)?;

        // Count the number of "page" tabs with "chrome://newtab/"
        let newtab_count = data.as_array().unwrap().iter().filter(|tab| {
            tab["type"] == "page" && (tab["url"] != "chrome://newtab/" || tab["url"] != "about:blank")
        }).count();
        let new_tab_pages: Vec<&Value> = data.as_array().unwrap().iter()
            .filter(|tab| tab["type"] == "page" && tab["url"] != "chrome://newtab/")
            .collect();

        for tab in &new_tab_pages {
            if let Some(url) = tab["url"].as_str() {
                log_message(&format!("URL: {}", url), "INFO");
            }
        }

        println!("");

        if newtab_count < 1 {
            let browser = browser.as_ref().unwrap();  // Access the Arc<Browser>
            // let current_tabs = browser.get_tabs().lock().unwrap().clone();

            // Open new tab with "chrome://newtab/"
            let new_tab = browser.new_tab()?;
            new_tab.navigate_to("chrome://newtab/")?;
            log_message("Ensured at least 1 'chrome://newtab/' pages are open.", "INFO");
        }
    }
}
