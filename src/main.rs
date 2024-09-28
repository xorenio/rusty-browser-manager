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
    // let tmp_browser = &browser.as_ref().unwrap(); // Access the Arc<Browser>
    loop {
        if browser.is_none() {
            match create_browser() {
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
        // if tmp_browser browser.is_some() {
        //     tmp_browser = browser.clone().as_ref().unwrap(); // Access the Arc<Browser>
        // }

        // Fetch the JSON data from the localhost endpoint.
        // let _response = get("http://127.0.0.1:9222/json")?;
        // let data: Value = serde_json::from_str(&response.text()?)?;

        // let external_tabs: Vec<&Value> = data
        //     .as_array()
        //     .unwrap()
        //     .iter()
        //     .filter(|tab| {
        //         tab["type"] == "page"
        //             && (tab["url"] != "chrome://newtab/" && tab["url"] != "about:blank")
        //     })
        //     .collect();
        // // // Collect the URLs of the open tabs
        // // let tab_urls: Vec<String> = external_tabs
        // //     .iter()
        // //     .filter_map(|tab| tab["url"].as_str().map(String::from))
        // //     .collect();

        // // // Wrap the list in an Arc<Mutex<_>>
        // // let tab_urls_arc = Arc::new(Mutex::new(tab_urls));

        // // // Let the user select a tab from the list
        // // if let Some(selected_index) = select_tab(Arc::clone(&tab_urls_arc)) {
        // //     let tab_urls_lock = tab_urls_arc.lock().unwrap();
        // //     println!("User selected tab: {}", tab_urls_lock[selected_index]);
        // // } else {
        // //     println!("No tab selected.");
        // // }

        // // Lock the tab_metadata to safely update it
        // let mut tab_metadata_lock = tab_metadata.lock().unwrap();

        // // Iterate over tab metadata and close those open for more than 5 minutes or already closed
        // tab_metadata_lock.retain(|tab_id, metadata| {
        //     // Check if the tab is still in the current list of open tabs
        //     let tab_still_open = external_tabs
        //         .iter()
        //         .any(|tab| tab["id"].as_str() == Some(tab_id));

        //     if !tab_still_open {
        //         log_message(&format!("Tab already closed: {}", tab_id), "INFO");
        //         return false; // Remove from metadata since it's already closed
        //     }

        //     // Get the current URL of the tab
        //     if let Some(tab_data) = external_tabs
        //         .iter()
        //         .find(|tab| tab["id"].as_str() == Some(tab_id))
        //     {
        //         if let Some(current_url) = tab_data["url"].as_str() {
        //             // If the URL has changed, update the metadata
        //             if current_url != metadata.current_url {
        //                 metadata.current_url = current_url.to_string();
        //                 metadata.last_url_change_time = Instant::now();
        //             }
        //         }
        //     }

        //     // Check if the tab has been on the same URL for more than 5 minutes
        //     if metadata.last_url_change_time.elapsed() > Duration::from_secs(300) {
        //         log_message(
        //             &format!("Closing tab (on same URL for over 5 minutes): {}", tab_id),
        //             "INFO",
        //         );
        //         if let Err(e) = metadata.tab.close_with_unload() {
        //             log_message(&format!("Failed to close tab: {}", e), "ERROR");
        //         }
        //         return false; // Remove closed tab from metadata
        //     }

        //     true // Keep tab in metadata if it's still open and URL hasn't stayed unchanged for 5 minutes
        // });

        // for tab in &external_tabs {
        //     if let Some(tab_id) = tab["id"].as_str() {
        //         if !tab_metadata_lock.contains_key(tab_id) {
        //             // If it's a new tab, add it to the metadata
        //             let browser_tabs = browser
        //                 .as_ref()
        //                 .expect("Browser instance not available")
        //                 .get_tabs()
        //                 .lock()
        //                 .unwrap();
        //             if let Some(new_tab) = browser_tabs
        //                 .iter()
        //                 .find(|tab| tab.get_target_id() == tab_id)
        //             {
        //                 if let Some(current_url) = tab["url"].as_str() {
        //                     tab_metadata_lock.insert(
        //                         tab_id.to_string(),
        //                         TabMetadata {
        //                             open_time: Instant::now(),
        //                             last_url_change_time: Instant::now(),
        //                             current_url: current_url.to_string(),
        //                             tab: Arc::clone(new_tab),
        //                         },
        //                     );
        //                 }
        //             }
        //         }
        //     }
        // }

        // for tab in &external_tabs {
        //     if let Some(url) = tab["url"].as_str() {
        //         log_message(&format!("URL: {}", url), "INFO");
        //     }
        // }
        // Count the number of "page" tabs with "chrome://newtab/"
        // let newtab_count = data
        //     .as_array()
        //     .unwrap()
        //     .iter()
        //     .filter(|tab| {
        //         tab["type"] == "page"
        //             && (tab["url"] == "chrome://newtab/" || tab["url"] == "about:blank")
        //     })
        //     .count();

        std::thread::sleep(std::time::Duration::from_nanos(1));
        let browser = browser.as_ref().unwrap(); // Access the Arc<Browser>
        let tabs = browser.get_tabs().lock().unwrap();
        // if tabs.len() > 2 {
        // Create an iterator for the tabs, starting from the second tab (index > 1)
        let mut tab_iter = tabs.iter().skip(1); // Skip the first tab (index 0)

        // Use a `while let` loop to activate each remaining tab
        while let Some(tab) = tab_iter.next() {
            let url = tab.get_url();
            if url == "chrome://newtab/" || url == "about:blank" {
                continue;
            }

            if url == "https://duckduckgo.com/" {
                continue;
            }
            // Check if the tab is still loading by executing JavaScript to get the document's readyState
            // let loading_check = tab.evaluate("document.readyState", true);
            // log_message(&format!("loading_check: {:?}", loading_check), "INFO");
            // let is_loading = if let Ok(result) = loading_check {
            //     // log_message(&format!("result.value: {:?}", result.value), "INFO");
            //     if let Some(ready_state) = result.value {
            //         ready_state.as_str() != Some("complete") // If it's not complete, we consider it loading
            //     } else {
            //         true // Assume loading if we can't retrieve the state
            //     }
            // } else {
            //     true // Assume loading on evaluation error
            // };
            // if is_loading {
            //     log_message(
            //         &format!("Tab is still loading, skipping: {}", tab.get_url()),
            //         "INFO",
            //     );
            //     continue;
            // }

            if let Err(e) = tab.bring_to_front() {
                log_message(&format!("Failed to bring to front tab: {}", e), "ERROR");
                if let Err(e) = tab.activate() {
                    log_message(&format!("Failed to activate tab: {}", e), "ERROR");
                }
            }
            // thread::sleep(Duration::from_nanos(1));
        }
        // } else {
        //     std::thread::sleep(std::time::Duration::from_millis(100)); // Delay for retry
        // }
    }
}
