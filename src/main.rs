use clap::Parser;
use console::{Emoji, style};
use curloo::{
    cli::{mode::CurlooMode, opts::Opts},
    ui,
};
use std::process::exit;

#[tokio::main]
async fn main() {
    let cli = Opts::parse();
    let mode = CurlooMode::from_string(cli.mode.as_ref().unwrap_or(&"auto".to_string()));

    match mode {
        Some(mode) => {
            if mode == CurlooMode::Auto {
                ui::run_auto_mode().await;
            } else {
                ui::run_manual_mode(cli).await;
            }
        }
        None => {
            eprintln!(
                "{} {}: {} mode does not exist",
                Emoji("❌", ""),
                style("curloo failed").red().bold(),
                cli.mode.as_ref().unwrap_or(&"unknown".to_string())
            );
            exit(1);
        }
    }
}
