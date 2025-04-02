use crate::request::Request;

/// Type alias for HTTP responses.
pub type Response = String;

/// Routes the request to the appropriate handler and returns a response.
pub fn route_request(request: &Request) -> Response {
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => respond_root(),
        ("GET", path) if path.starts_with("/echo/") => respond_echo(path),
        _ => respond_not_found(),
    }
}

fn respond_root() -> Response {
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

fn respond_echo(path: &str) -> Response {
    let echoed_message = path.trim_start_matches("/echo/");
    let content_length = echoed_message.len();
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content_length, echoed_message
    )
}

fn respond_not_found() -> Response {
    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}
