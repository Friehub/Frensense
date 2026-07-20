// SAFE: Stripped ANSI escape codes and control characters from user input before logging.

use log::{info, warn};
use regex::Regex;

fn strip_ansi(s: &str) -> String {
    let re = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    re.replace_all(s, "").to_string()
}

fn strip_control(s: &str) -> String {
    s.chars().filter(|&c| c.is_ascii_graphic() || c == ' ').collect()
}

fn handle_login(username: &str, ip: &str) {
    let safe_username = strip_control(&strip_ansi(username));
    info!("User {} logged in from {}", safe_username, ip);
}

fn process_feedback(user_input: &str) {
    let safe_input = strip_control(&strip_ansi(user_input));
    info!("User feedback: {}", safe_input);
}
