use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum MyError {
    UnsupportedFormat(String),
    UnsupportedHttpMethod(String),
    ParseError(String),
    RequestError(String),         // For errors during HTTP requests
    InvalidUrl(String),           // For invalid URLs
    InvalidInput(String),         // For invalid user input
    IoError(String),              // For I/O-related errors
    SerializationError(String),   // For serialization errors
    DeserializationError(String), // For deserialization errors
    UnknownError(String),         // For unexpected errors
    ResponseError(String),        // For response failures
    ClientError(String),          // When failing to create HTTP client
    MissingRequestBody,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::UnsupportedHttpMethod(method) => {
                write!(f, "'{}' is not a supported HTTP method.", method)
            }
            MyError::UnsupportedFormat(format) => {
                write!(f, "'{}' is not a supported format.", format)
            }
            MyError::ParseError(e) => {
                write!(f, "Parse error: {}", e)
            }
            MyError::RequestError(e) => {
                write!(f, "Request error: {}", e)
            }
            MyError::InvalidUrl(url) => {
                write!(f, "Invalid URL: {}", url)
            }
            MyError::InvalidInput(input) => {
                write!(f, "Invalid input: {}", input)
            }
            MyError::IoError(e) => {
                write!(f, "I/O error: {}", e)
            }
            MyError::SerializationError(e) => {
                write!(f, "Serialization error: {}", e)
            }
            MyError::DeserializationError(e) => {
                write!(f, "Deserialization error: {}", e)
            }
            MyError::UnknownError(e) => {
                write!(f, "Unknown error: {}", e)
            }
            MyError::ResponseError(e) => {
                write!(f, "Response error: {}", e)
            }
            MyError::ClientError(e) => {
                write!(f, "Reponse error: {}", e)
            }
            MyError::MissingRequestBody => write!(f, "Missing body"),
        }
    }
}

impl std::error::Error for MyError {}
