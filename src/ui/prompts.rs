use crate::{
    http::{handler::HttpMethod, validation},
    models::Header,
};
use console::{Emoji, style};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use serde_json::{Map, Value as JsonValue};
use std::str::FromStr;

pub fn select_http_method() -> HttpMethod {
    println!(
        "{}",
        console::style("STEP 1: Choose HTTP Method").cyan().bold()
    );
    let methods = &["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"];
    let method_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select HTTP method")
        .default(0)
        .items(&methods[..])
        .interact()
        .unwrap();

    HttpMethod::from_str(methods[method_index]).unwrap()
}

pub fn collect_url_input() -> String {
    println!(
        "{}",
        console::style("STEP 2: Enter Request URL").cyan().bold()
    );
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("URL")
        .validate_with(|input: &String| -> Result<(), &str> {
            if validation::validate_url(input).is_err() {
                return Err("URL must start with http:// or https://");
            }
            Ok(())
        })
        .interact_text()
        .unwrap()
}

pub async fn collect_optional_headers() -> Vec<Header> {
    println!(
        "{}",
        console::style("STEP 3: Add Headers (optional)")
            .cyan()
            .bold()
    );
    let add_headers = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to add headers?")
        .default(1)
        .items(&["Yes", "No"])
        .interact()
        .unwrap();

    if add_headers == 0 {
        collect_headers().await
    } else {
        vec![]
    }
}

pub async fn collect_headers() -> Vec<Header> {
    let mut headers = Vec::new();

    loop {
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Header name (leave empty to finish)")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if name.is_empty() {
            break;
        }

        let value: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Header value")
            .interact_text()
            .unwrap();

        headers.push(Header { name, value });
    }

    headers
}

pub fn select_response_format() -> &'static str {
    let formats = &["json", "yaml", "text"];

    println!(
        "{}",
        console::style("STEP 4: Choose Response Format")
            .cyan()
            .bold()
    );
    let format_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select format")
        .default(0)
        .items(&formats[..])
        .interact()
        .unwrap();

    formats[format_index]
}
pub async fn prompt_for_body() -> Option<JsonValue> {
    println!(
        "{}",
        style("STEP 4: Enter Request Body (Key-Value Pairs)")
            .cyan()
            .bold()
    );

    let mut body = Map::new();

    loop {
        let key: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Key (e.g., 'user.name' for nested JSON, leave empty to finish)")
            .allow_empty(true)
            .interact_text()
            .unwrap();

        if key.is_empty() {
            break;
        }

        let value: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Value (e.g., 42, true, \"text\", [1, 2, 3], {\"key\": \"value\"})")
            .interact_text()
            .unwrap();

        let json_value = infer_json_value(&value);

        // "user.name" -> ["user", "name"] - nested json
        let keys: Vec<&str> = key.split('.').collect();

        let mut current = &mut body;
        for (i, key_part) in keys.iter().enumerate() {
            if i == keys.len() - 1 {
                current.insert(key_part.to_string(), json_value.clone());
            } else {
                let entry = current
                    .entry(key_part.to_string())
                    .or_insert(JsonValue::Object(Map::new()));
                if let JsonValue::Object(map) = entry {
                    current = map;
                } else {
                    eprintln!(
                        "{} {}: Key '{}' already exists and is not an object.",
                        Emoji("❌", ""),
                        style("Error").red().bold(),
                        key_part
                    );
                    break;
                }
            }
        }
    }

    if body.is_empty() {
        None
    } else {
        Some(JsonValue::Object(body))
    }
}

fn infer_json_value(value: &str) -> JsonValue {
    if let Ok(number) = value.parse::<i64>() {
        return JsonValue::Number(number.into());
    }
    if let Ok(number) = value.parse::<f64>() {
        return JsonValue::Number(serde_json::Number::from_f64(number).unwrap());
    }

    if let Ok(boolean) = value.parse::<bool>() {
        return JsonValue::Bool(boolean);
    }

    if let Ok(json) = serde_json::from_str::<JsonValue>(value) {
        return json;
    }

    JsonValue::String(value.to_string())
}

