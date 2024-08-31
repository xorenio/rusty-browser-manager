use std::time::Duration;
use std::thread::sleep;
use reqwest::blocking::get;
use anyhow::Result;
use serde_json::Value;
use headless_chrome::{Browser, LaunchOptions};
use std::thread;

fn main() -> Result<()> {
    loop {
        // Fetch the JSON data from the localhost endpoint.
        let response = get("http://localhost:9222/json")?;
        let data: Value = serde_json::from_str(&response.text()?)?;

        // Find all tabs with type "page" and url "chrome://newtab/"
        let new_tab_pages: Vec<&Value> = data.as_array().unwrap().iter()
            .filter(|tab| tab["type"] == "page" && tab["url"] == "chrome://newtab/")
            .collect();

        // Check if there are at least 2 "chrome://newtab/" tabs.
        if new_tab_pages.len() >= 2 {
            println!("Found at least 2 'chrome://newtab/' tabs.");

            // Optionally, you can do something with these tabs.
            // Here, we connect to the first found tab as an example.
            let tab_info = new_tab_pages[0];
            let tab_id = tab_info["id"].as_str().unwrap();
            let ws_url = format!("ws://127.0.0.1:9222/devtools/page/{}", tab_id);

            // Connect to the browser using the found WebSocket URL.
            let browser = Browser::connect(ws_url)?;
            println!("Connected to browser with tab id: {}", tab_id);

            // Perform actions on the tab using the `browser` object.
            let tab = browser.wait_for_initial_tab()?;
            tab.navigate_to("https://chatgpt.com")?;
            tab.wait_for_element("body")?;
            thread::sleep(Duration::from_secs(20));


            tab.navigate_to("https://www.gitlab.com")?;
            tab.wait_for_element("body")?;
            thread::sleep(Duration::from_secs(20));

            tab.navigate_to("https://www.github.com")?;
            tab.wait_for_element("body")?;
            thread::sleep(Duration::from_secs(20));


            thread::sleep(Duration::from_secs(120));

            // Close the tab after work is done.
            tab.close(true)?;
            break;
        } else {
            println!("Less than 2 'chrome://newtab/' tabs found, retrying in 10 seconds...");
            sleep(Duration::from_secs(10));
        }
    }

    Ok(())
}
