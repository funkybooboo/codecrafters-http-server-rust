use std::collections::HashMap;
use std::io::Error;
use crate::request::Request;
use crate::response::Response;

pub type Route = Box<dyn for<'a, 'b> Fn(&'a mut Request, &'b mut Response) + Send + Sync + 'static>;

pub struct RouteEntry {
    pub method: String,
    pub pattern: String,
    pub handler: Route,
}

pub struct Router {
    pub routes: Vec<RouteEntry>,
    pub default_route: Route,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            default_route: Box::new(not_found_route),
        }
    }

    /// Registers a new route with its method and pattern.
    pub fn register(&mut self, method: &str, pattern: &str, handler: Route) {
        self.routes.push(RouteEntry {
            method: method.to_string(),
            pattern: pattern.to_string(),
            handler,
        });
    }

    pub fn route(&self, request: &mut Request) -> Response {
        let mut response = Response::new();
        for entry in &self.routes {
            if entry.method.eq_ignore_ascii_case(&request.method) {
                if let Some(params) = match_route(&entry.pattern, &request.path) {
                    request.params = params;
                    (entry.handler)(request, &mut response);
                    return response;
                }
            }
        }
        (self.default_route)(request, &mut response);
        response
    }
}

fn match_route(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let pattern_segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if pattern_segments.len() != path_segments.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (p_seg, path_seg) in pattern_segments.iter().zip(path_segments.iter()) {
        if p_seg.starts_with('{') && p_seg.ends_with('}') {
            let key = p_seg.trim_matches(|c| c == '{' || c == '}');
            params.insert(key.to_string(), (*path_seg).to_string());
        } else if p_seg != path_seg {
            return None;
        }
    }
    Some(params)
}

pub fn not_found_route(_req: &mut Request, res: &mut Response) {
    res.status_code = 404;
    res.status_text = "Not Found".to_string();
    res.body = "The requested resource was not found.".to_string();
}

pub fn make_interval_server_error_route(e: Error) -> Route {
    Box::new(move |_req: &mut Request, res: &mut Response| {
        res.status_code = 500;
        res.status_text = "Internal Server Error".to_string();
        res.body = format!("Error: {}", e)
    })
}
