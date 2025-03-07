use crate::errors::myerror::MyError;
use reqwest::Client;


pub fn create_client() -> Result<Client, MyError> {
    Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("curloo/0.1") 
        .build()
        .map_err(|e| MyError::ClientError(e.to_string()))
}

pub async fn get(client: &Client, url: &str) -> Result<String, MyError> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| MyError::RequestError(e.to_string()))?;

    response
        .text()
        .await
        .map_err(|e| MyError::ResponseError(e.to_string()))
}

pub async fn post_json(
    client: &Client,
    url: &str,
    body: &serde_json::Value,
) -> Result<String, MyError> {
    let response = client
        .post(url)
        .json(body)
        .send()
        .await
        .map_err(|e| MyError::RequestError(e.to_string()))?;

    response
        .text()
        .await
        .map_err(|e| MyError::ResponseError(e.to_string()))
}

pub fn add_default_headers(request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    request
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
}
