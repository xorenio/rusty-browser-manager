mod utils;

use utils::log_message;
use std::time::Duration;
use std::thread::sleep;
use reqwest::blocking::get;
use anyhow::Result;
use serde_json::Value;
use headless_chrome::Browser;
use std::thread;

fn main() -> Result<()> {
    loop {
        // Fetch the JSON data from the localhost endpoint.
        let response = match get("http://localhost:9222/json") {
            Ok(resp) => resp,
            Err(err) => {
                log_message(&format!("Failed to fetch JSON data: {}", err), "ERROR");
                sleep(Duration::from_secs(10));
                continue;
            }
        };

        let response_text = match response.text() {
            Ok(text) => text,
            Err(err) => {
                log_message(&format!("Failed to read response text: {}", err), "ERROR");
                sleep(Duration::from_secs(10));
                continue;
            }
        };

        let data: Value = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(err) => {
                log_message(&format!("Failed to parse JSON data: {}", err), "ERROR");
                sleep(Duration::from_secs(10));
                continue;
            }
        };

        // Find all tabs with type "page" and url "chrome://newtab/"
        let new_tab_pages: Vec<&Value> = match data.as_array() {
            Some(array) => array.iter()
                .filter(|tab| tab["type"] == "page" && tab["url"] == "chrome://newtab/")
                .collect(),
            None => {
                log_message(&format!("Data is not an array"), "ERROR");
                sleep(Duration::from_secs(10));
                continue;
            }
        };

        // Check if there are at least 2 "chrome://newtab/" tabs.
        if new_tab_pages.len() >= 1 {
            log_message(&format!("Found at least 1 'chrome://newtab/' tabs."), "INFO");

            // Optionally, you can do something with these tabs.
            // Here, we connect to the first found tab as an example.
            let tab_info = &new_tab_pages[0];
            let tab_id = match tab_info["id"].as_str() {
                Some(id) => id,
                None => {
                    log_message(&format!("Tab ID is missing or not a string"), "ERROR");
                    sleep(Duration::from_secs(10));
                    continue;
                }
            };

            let ws_url = &format!("ws://127.0.0.1:9222/devtools/page/{}", tab_id);

            // Connect to the browser using the found WebSocket URL.
            let browser = match Browser::connect(ws_url.clone()) {
                Ok(browser) => browser,
                Err(err) => {
                    log_message(&format!("Failed to connect to browser: {}", err), "ERROR");
                    sleep(Duration::from_secs(10));
                    continue;
                }
            };
            log_message(&format!("Connected to browser with tab id: {}", tab_id), "INFO");

            // Perform actions on the tab using the `browser` object.
            let tab = match browser.new_tab() {
                Ok(tab) => tab,
                Err(err) => {
                    log_message(&format!("Failed to create a new tab: {}", err), "ERROR");
                    sleep(Duration::from_secs(10));
                    continue;
                }
            };

            // Set the user agent, accept language, and platform for this specific tab.
            let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
            let accept_language = Some("en-US,en;q=0.9");
            let platform = Some("Win32");

            // Set the user agent for this specific tab
            if let Err(err) = tab.set_user_agent(user_agent, accept_language, platform) {
                log_message(&format!("Failed to set user agent: {}", err), "ERROR");
                tab.close_target()?;
                sleep(Duration::from_secs(10));
                continue;
            }

            if let Err(err) = tab.navigate_to("https://chatgpt.com") {
                log_message(&format!("Failed to navigate to ChatGPT: {}", err), "ERROR");
                tab.close_target()?;
                sleep(Duration::from_secs(10));
                continue;
            }
            tab.wait_for_element("body")?;
            thread::sleep(Duration::from_secs(20));

            // Close the tab after work is done.
            if let Err(err) = tab.close_target() {
                log_message(&format!("Failed to close tab: {}", err), "ERROR");
            }
            break;
        } else {
            println!("Less than 2 'chrome://newtab/' tabs found, retrying in 10 seconds...");
            sleep(Duration::from_secs(10));
        }
    }

    Ok(())
}
