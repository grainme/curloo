use chrono::Local;
use console::{Emoji, style};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn save_response_to_file(content: &str, format: &str, method: &str, url: &str) {
    let responses_dir = "responses";
    if !Path::new(responses_dir).exists() {
        fs::create_dir(responses_dir).expect("Failed to create responses directory");
    }

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let url_hash = format!("{:x}", md5::compute(url));
    let filename = format!(
        "{}/response_{}_{}_{}.{}",
        responses_dir, timestamp, method, url_hash, format
    );

    let mut file = File::create(&filename).expect("Failed to create file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to file");

    println!(
        "{} {}",
        Emoji("📁", ""),
        style(format!("Response saved to {}", filename))
            .green()
            .bold()
    );
}
