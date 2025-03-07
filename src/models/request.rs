use crate::http::handler::HttpMethod; 
use serde_json::Value as JsonValue;

pub struct Request {
    pub url: String,
    pub method: HttpMethod,
    pub body: Option<JsonValue>,
    pub headers: Vec<crate::models::Header>,
}

impl Request {
    pub fn new(url: String, method: HttpMethod) -> Self {
        Self {
            url,
            method,
            body: None,
            headers: Vec::new(),
        }
    }

    pub fn with_headers(
        url: String,
        method: HttpMethod,
        body: Option<JsonValue>,
        headers: Vec<crate::models::Header>,
    ) -> Self {
        Self {
            url,
            method,
            body,
            headers,
        }
    }
}
