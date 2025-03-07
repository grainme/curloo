use crate::{
    http::{client::create_client, handler::HttpMethod},
    models::{request::Request, response::HttpResponse, Header},
    ui::{prompts::select_response_format, spinner::create_progress_bar},
    utils::{file_io::save_response_to_file, formatter::format_response_body},
};
use console::{Emoji, style};
use reqwest::Client;
use tabled::{Table, settings::Style as TableStyle};
use serde_json::Value as JsonValue;

const LOGO: &str = r#"
   ___ _   _ ___ _    ___  ___
  / __| | | | _ \ |  / _ \/ _ \
 | (__| |_| |   / |_| (_) | (_) |
  \___|\___/|_|_\____\___/ \___/
"#;

pub fn print_welcome() {
    println!("{}", style(LOGO).cyan().bold());

    println!(
        "{} {} {}",
        Emoji("👋", ""),
        style("Welcome to").green().bold(),
        style("curloo").cyan().bold(),
    );

    println!(
        "{} {} {}",
        style("Version:").dim(),
        style("0.1.0").green().bold(),
        style("| Your friendly HTTP client").dim(),
    );

    println!("{}", style("~~~".repeat(20)).dim());
}

pub fn print_divider() {
    println!("\n{}\n", "─".repeat(50));
}

pub fn display_request_summary(method: &HttpMethod, url: &str, headers: &[Header]) {
    println!("{}", style("REQUEST SUMMARY").cyan().bold());

    let method_str = method.to_string();
    let method_style = match method_str.as_str() {
        "GET" => style(method_str).green().bold(),
        "POST" => style(method_str).blue().bold(),
        "PUT" => style(method_str).yellow().bold(),
        "DELETE" => style(method_str).red().bold(),
        "HEAD" => style(method_str).magenta().bold(),
        _ => style(method_str).magenta().bold(),
    };

    println!("{} {}", style("Method:").dim(), method_style);
    println!("{} {}", style("URL:   ").dim(), style(url).green());

    if !headers.is_empty() {
        println!("\n{}", style("Headers:").dim());
        println!("{}", Table::new(headers).with(TableStyle::rounded()));
    }
}

pub fn display_response_summary(response: &HttpResponse) {
    println!("{}", style("RESPONSE SUMMARY").cyan().bold());
    let status_style = match response.status {
        s if (200..=300).contains(&s) => style(format!("{}", s)).green().bold(),
        s if (300..=400).contains(&s) => style(format!("{}", s)).yellow().bold(),
        s if (400..=500).contains(&s) => style(format!("{}", s)).red().bold(),
        s => style(format!("{}", s)).white().on_red().bold(),
    };

    println!("{} {}", style("Status:").dim(), status_style);
    println!(
        "{} {}",
        style("Time:  ").dim(),
        style(format!("{:.2}s", response.elapsed.as_secs_f64())).dim()
    );
    println!(
        "{} {}",
        style("Size:  ").dim(),
        style(format!(
            "{:.2} KB",
            response.content_length() as f64 / 1024.0
        ))
        .dim()
    );
}

pub fn display_response_headers(response: &HttpResponse) {
    println!("\n{}", style("Headers:").dim());
    println!(
        "{}",
        Table::new(&response.headers).with(TableStyle::rounded())
    );
}

pub fn show_manual_mode_message() {
    println!("{}", style("Manual mode is coming soon!").yellow().bold());
    println!(
        "Try using auto mode with: {} curloo --mode auto",
        style("cargo run --").dim()
    );
}

pub async fn execute_auto_mode(method: HttpMethod, url: String, body: Option<JsonValue>, headers: Vec<Header>) {
    let client: Client = create_client().expect("curloo error: failed to instantiate a client.");

    display_request_summary(&method, &url, &headers);
    print_divider();

    let pb = create_progress_bar(&method.to_string(), &url);

    let request = Request::with_headers(url.clone(), method.clone(), body, headers);
    let response = match HttpMethod::handle_request(&request, client.clone()).await {
        Ok(response) => response,
        Err(e) => {
            pb.finish_and_clear();
            eprintln!(
                "{} {}",
                Emoji("❌", ""),
                style(format!("Request failed: {}", e)).red().bold()
            );
            std::process::exit(1);
        }
    };

    pb.finish_and_clear();
    println!(
        "{} {}",
        Emoji("✓", ""),
        style(format!(
            "Request completed in {:.2}s",
            response.elapsed.as_secs_f64()
        ))
        .green()
        .bold()
    );
    print_divider();

    let format = select_response_format();
    print_divider();

    display_response_summary(&response);

    let formatted_body = format_response_body(&response.body, format);

    save_response_to_file(&formatted_body, format, &method.to_string(), &url);

    print_divider();

    display_response_headers(&response);
}