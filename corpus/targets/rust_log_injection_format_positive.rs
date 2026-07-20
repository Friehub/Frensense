// [frensense]
// observation: User-controlled input is passed to log::info!() or similar macros, allowing ANSI escape code injection into log output.
// impact: An attacker can inject ANSI escape sequences that corrupt log files, hide malicious entries, or exploit terminal emulator vulnerabilities.
// improvement: Strip ANSI escape codes or shell special characters from user input before logging.

use log::{info, warn};

fn handle_login(username: &str, ip: &str) {
    info!("User {} logged in from {}", username, ip);
}

fn process_feedback(user_input: &str) {
    info!("User feedback: {}", user_input);
}
