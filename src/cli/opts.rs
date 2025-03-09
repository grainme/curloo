use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "curloo")]
#[command(version = "1.0.0")]
#[command(
    about = "A friendly HTTP client for testing REST APIs.",
    long_about = "Curloo is a lightweight HTTP client that combines command-line efficiency with interactive functionality. \
                  It supports both interactive (auto) and direct (manual) modes."
)]
pub struct Opts {
    #[arg(
        help = "Operation mode: 'auto' for interactive prompts, 'manual' for direct command input.",
        value_parser = ["auto", "manual"],
        default_value = "manual",
        long
    )]
    pub mode: Option<String>,

    #[arg(
        help = "HTTP request method. Defaults to GET if not specified.",
        value_parser = ["get", "post", "delete", "put"],
        default_value = "get",
        short = 'm',
        long
    )]
    pub method: Option<String>,

    #[arg(long, short = 'u', help = "Target URL fot the HTTP request.")]
    pub url: Option<String>,

    #[arg(long, short = 'b', help = "Request body, typically used with POST and PUT requests.")]
    pub body: Option<String>,
}
