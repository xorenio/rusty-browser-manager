// utils.rs

use chrono;

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
