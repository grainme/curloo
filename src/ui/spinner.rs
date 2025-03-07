use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

// Constants
const SPINNER_CHARS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn create_progress_bar(method: &str, url: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(SPINNER_CHARS)
            .template("{spinner} {msg}")
            .expect("Failed to set progress bar template"),
    );
    pb.set_message(format!("Making {} request to {}...", method, url));
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}
