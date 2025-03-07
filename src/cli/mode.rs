use clap::Parser;

#[derive(Debug, Parser, Clone, PartialEq, Eq)]
pub enum CurlooMode {
    Auto,
    Manual,
}

impl CurlooMode {
    pub fn from_string(mode: &str) -> Option<Self> {
        match mode.to_ascii_lowercase().trim() {
            "manual" => Some(Self::Manual),
            "auto" => Some(Self::Auto),
            _ => None,
        }
    }
}
