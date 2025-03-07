use crate::errors::myerror::MyError;
use crate::models::Header;
use crate::models::request::Request;
use crate::models::response::HttpResponse;
use reqwest::Client;
use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use super::client::add_default_headers;

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
}

impl FromStr for HttpMethod {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            _ => Err(format!("Unknown HTTP method: {}", s)),
        }
    }
}

impl HttpMethod {
    pub async fn handle_request(
        request: &Request,
        client: Client,
    ) -> Result<HttpResponse, MyError> {
        let start = Instant::now();

        let body = request.body.as_ref().unwrap_or(&serde_json::Value::Null);

        let mut req_builder = match request.method {
            HttpMethod::GET => client.get(&request.url),
            HttpMethod::POST => client.post(&request.url),
            HttpMethod::PUT => client.put(&request.url),
            HttpMethod::DELETE => client.delete(&request.url),
            HttpMethod::PATCH => client.patch(&request.url),
            HttpMethod::HEAD => client.head(&request.url),
        };

        if body != &serde_json::Value::Null {
            req_builder = req_builder.json(body);
        }

        for header in &request.headers {
            req_builder = req_builder.header(&header.name, &header.value);
        }

        let req_builder = add_default_headers(req_builder);

        let response = req_builder
            .send()
            .await
            .map_err(|e| MyError::RequestError(e.to_string()))?;
        let status = response.status().as_u16();

        let mut headers = Vec::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.push(Header {
                    name: name.to_string(),
                    value: value_str.to_string(),
                });
            }
        }

        let body = response
            .text()
            .await
            .map_err(|e| MyError::ResponseError(e.to_string()))?;
        let elapsed = start.elapsed();

        Ok(HttpResponse::with_headers(status, body, headers, elapsed))
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::HEAD => write!(f, "HEAD"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;
    use std::time::Duration;

    #[tokio::test]
    async fn test_handle_request_get() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "success"}"#)
            .create_async()
            .await;

        let request = Request {
            method: HttpMethod::GET,
            url: server.url(),
            body: None,
            headers: vec![],
        };

        let client = Client::new();
        let response = HttpMethod::handle_request(&request, client).await.unwrap();

        assert_eq!(response.status, 200);
        assert_eq!(response.body, r#"{"message": "success"}"#);
        assert!(response.elapsed > Duration::from_secs(0));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_handle_request_post_with_body() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 1}"#)
            .create_async()
            .await;

        let request = Request {
            method: HttpMethod::POST,
            url: server.url(),
            body: Some(json!({ "name": "graine user" })),
            headers: vec![Header {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            }],
        };

        let client = Client::new();
        let response = HttpMethod::handle_request(&request, client).await.unwrap();

        assert_eq!(response.status, 201);
        assert_eq!(response.body, r#"{"id": 1}"#);
        assert!(response.elapsed > Duration::from_secs(0));

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_handle_request_error() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("GET", "/")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "not found"}"#)
            .create_async()
            .await;

        let request = Request {
            method: HttpMethod::GET,
            url: server.url(),
            body: None,
            headers: vec![],
        };

        let client = Client::new();
        let response = HttpMethod::handle_request(&request, client).await.unwrap();

        assert_eq!(response.status, 404);
        assert_eq!(response.body, r#"{"error": "not found"}"#);
        assert!(response.elapsed > Duration::from_secs(0));

        mock.assert_async().await;
    }
}
