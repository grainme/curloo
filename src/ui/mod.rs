use display::{print_divider, print_welcome};
use prompts::{collect_optional_headers, collect_url_input, prompt_for_body, select_http_method};
use crate::{cli::opts::Opts, http::handler::HttpMethod, models::Header};

mod display;
mod prompts;
mod spinner;

pub async fn run_auto_mode() {
    print_welcome();

    let method: HttpMethod = select_http_method();
    print_divider();

    let url: String = collect_url_input();
    print_divider();

    let headers: Vec<Header> = collect_optional_headers().await;
    print_divider();

    let body = prompt_for_body().await;

    display::execute_auto_mode(method, url, body, headers).await;
}

pub async fn run_manual_mode(_cli: Opts) {
    display::show_manual_mode_message();
}
