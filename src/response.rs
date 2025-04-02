use std::collections::HashMap;

/// HTTP Response struct.
pub struct Response {
    pub http_version: String,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    /// Creates a new empty Response with default values.
    pub fn new() -> Self {
        Self {
            http_version: "HTTP/1.1".to_string(),
            status_code: 200,
            status_text: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    /// Converts the Response struct into a complete HTTP response string.
    pub fn format_response(&self) -> String {
        let mut response = format!("{} {} {}\r\n", self.http_version, self.status_code, self.status_text);
        for (header, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", header, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}
