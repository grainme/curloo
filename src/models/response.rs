pub struct HttpResponse {
    pub status: u16,
    pub body: String,
    pub headers: Vec<crate::models::Header>,
    pub elapsed: std::time::Duration,
}

impl HttpResponse {
    pub fn new(status: u16, body: String, elapsed: std::time::Duration) -> Self {
        Self {
            status,
            body,
            headers: Vec::new(),
            elapsed,
        }
    }

    pub fn with_headers(
        status: u16,
        body: String,
        headers: Vec<crate::models::Header>,
        elapsed: std::time::Duration,
    ) -> Self {
        Self {
            status,
            body,
            headers,
            elapsed,
        }
    }

    pub fn content_length(&self) -> usize {
        self.body.len()
    }
}
