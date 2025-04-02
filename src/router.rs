use crate::request::Request;
use std::collections::HashMap;

/// HTTP Response struct.
pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {
    /// Creates a new empty Response with default values.
    pub fn new() -> Self {
        Self {
            status_code: 200,
            status_text: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    /// Converts the Response struct into a complete HTTP response string.
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

/// Unified route handler type.
/// Each handler receives a mutable reference to the Request (which will have any
/// matched parameters stored in `req.params`) and a mutable reference to the Response.
pub type Route = fn(&mut Request, &mut Response);

/// Router holds all registered routes in a single map and a default (not found) handler.
pub struct Router {
    routes: HashMap<String, Route>,
    default_route: Route,
}

impl Router {
    /// Creates a new Router with the default not-found handler.
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            default_route: not_found_route,
        }
    }

    /// Registers a route with a given pattern and handler.
    /// The pattern can include dynamic segments (e.g. "/echo/{msg}").
    pub fn register(&mut self, pattern: &str, handler: Route) {
        self.routes.insert(pattern.to_string(), handler);
    }

    /// Routes the request.
    /// Iterates over all registered routes. If a route pattern matches the request path,
    /// the extracted parameters are stored in `request.params` and the corresponding handler is invoked.
    /// If no route matches, the default handler is used.
    pub fn route(&self, request: &mut Request) -> Response {
        let mut response = Response::new();

        for (pattern, handler) in &self.routes {
            if let Some(params) = match_route(pattern, &request.path) {
                request.params = params;
                handler(request, &mut response);
                return response;
            }
        }
        (self.default_route)(request, &mut response);
        response
    }
}

/// Matches a route pattern (e.g. "/echo/{msg}") against a request path.
/// Returns a map of extracted parameters if the pattern matches, otherwise None.
fn match_route(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let pattern_segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if pattern_segments.len() != path_segments.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (p_seg, path_seg) in pattern_segments.iter().zip(path_segments.iter()) {
        if p_seg.starts_with('{') && p_seg.ends_with('}') {
            // Extract parameter name (e.g. "{msg}" becomes "msg")
            let key = p_seg.trim_matches(|c| c == '{' || c == '}');
            params.insert(key.to_string(), (*path_seg).to_string());
        } else if p_seg != path_seg {
            return None;
        }
    }
    Some(params)
}

// ------------------- Example Handlers -------------------

/// Handler for the root route (static).
fn root_route(_req: &mut Request, res: &mut Response) {
    res.status_code = 200;
    res.status_text = "OK".to_string();
}

/// Handler for the echo route (dynamic).
/// Expects the pattern "/echo/{msg}" so that "msg" is extracted into req.params.
fn echo_route(req: &mut Request, res: &mut Response) {
    let msg = req.params.get("msg").expect("Expected parameter 'msg'");
    res.status_code = 200;
    res.status_text = "OK".to_string();
    res.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    res.headers.insert("Content-Length".to_string(), msg.len().to_string());
    res.body = msg.clone();
}

/// Default handler for unmatched routes.
fn not_found_route(_req: &mut Request, res: &mut Response) {
    res.status_code = 404;
    res.status_text = "Not Found".to_string();
    res.body = "The requested resource was not found.".to_string();
}

/// Convenience function to create a router with pre-registered routes.
pub fn create_router() -> Router {
    let mut router = Router::new();
    router.register("/", root_route);
    router.register("/echo/{msg}", echo_route);
    router
}
