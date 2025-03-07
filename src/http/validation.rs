pub fn validate_url(url: &str) -> Result<(), String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(())
    } else {
        Err("URL must start with http:// or https://".to_string())
    }
}

pub fn validate_header_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Header name cannot be empty".to_string());
    }

    if name.contains(":") {
        return Err("Header name cannot contain colon character".to_string());
    }

    Ok(())
}
