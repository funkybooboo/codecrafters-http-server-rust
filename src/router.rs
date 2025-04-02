use std::collections::HashMap;
use crate::request::Request;
use crate::response::Response;

pub type Route = fn(&mut Request, &mut Response);

pub struct Router {
    routes: HashMap<String, Route>,
    default_route: Route,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            default_route: not_found_route,
        }
    }

    pub fn register(&mut self, pattern: &str, handler: Route) {
        self.routes.insert(pattern.to_string(), handler);
    }

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

fn not_found_route(_req: &mut Request, res: &mut Response) {
    res.status_code = 404;
    res.status_text = "Not Found".to_string();
    res.body = "The requested resource was not found.".to_string();
}
