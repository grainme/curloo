use console::style;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

pub fn format_response_body(body: &str, format: &str) -> String {
    match format {
        "json" => match serde_json::from_str::<JsonValue>(body) {
            Ok(json) => serde_json::to_string_pretty(&json).unwrap(),
            Err(_) => {
                println!("{}", style("Not valid JSON. Showing as text.").yellow());
                body.to_string()
            }
        },
        "yaml" => match serde_yaml::from_str::<YamlValue>(body) {
            Ok(yaml) => serde_yaml::to_string(&yaml).unwrap(),
            Err(_) => {
                eprintln!("{}", style("Not valid YAML. Showing as text.").yellow());
                body.to_string()
            }
        },
        "text" => body.to_string(),
        _ => unreachable!(),
    }
}

pub fn get_content_type(format: &str) -> &'static str {
    match format {
        "json" => "application/json",
        "yaml" => "application/yaml",
        _ => "text/plain",
    }
}
