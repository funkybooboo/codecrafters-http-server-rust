use crate::request::Request;

/// HTTP Response struct.
pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    /// Creates a new Response.
    pub fn new(
        status_code: u16,
        status_text: &str,
        headers: Vec<(String, String)>,
        body: &str,
    ) -> Self {
        Self {
            status_code,
            status_text: status_text.to_string(),
            headers,
            body: body.to_string(),
        }
    }

    /// Converts the Response struct into a formatted HTTP response string.
    pub fn format_response(&self) -> String {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        for (header, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", header, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}

/// Routes the request to the appropriate route and returns a Response.
pub fn router(request: &Request) -> Response {
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => root_route(),
        ("GET", path) if path.starts_with("/echo/") => echo_route(path),
        _ => not_found_route(),
    }
}

fn root_route() -> Response {
    Response::new(200, "OK", vec![], "")
}

fn echo_route(path: &str) -> Response {
    let echoed_message = path.trim_start_matches("/echo/");
    let content_length = echoed_message.len().to_string();
    Response::new(
        200,
        "OK",
        vec![
            ("Content-Type".to_string(), "text/plain".to_string()),
            ("Content-Length".to_string(), content_length),
        ],
        echoed_message,
    )
}

fn not_found_route() -> Response {
    Response::new(404, "Not Found", vec![], "")
}
