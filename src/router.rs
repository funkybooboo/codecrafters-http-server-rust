use std::collections::HashMap;
use crate::request::Request;
use crate::response::Response;

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

/// Default handler for unmatched routes.
pub fn not_found_route(_req: &mut Request, res: &mut Response) {
    res.status_code = 404;
    res.status_text = "Not Found".to_string();
    res.body = "The requested resource was not found.".to_string();
}
