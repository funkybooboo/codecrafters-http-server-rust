use crate::request::Request;
use crate::response::Response;
use crate::router::{make_interval_server_error_route, Route};
use std::fs;
use std::path::{Path, PathBuf};
use crate::router::not_found_route;

pub fn root_route(_req: &mut Request, res: &mut Response) {
    res.status_code = 200;
    res.status_text = "OK".to_string();
}

pub fn echo_route(req: &mut Request, res: &mut Response) {
    let msg = req.params.get("msg").expect("Expected parameter 'msg'");
    res.status_code = 200;
    res.status_text = "OK".to_string();
    res.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    res.headers.insert("Content-Length".to_string(), msg.len().to_string());
    res.body = msg.clone();
}

pub fn user_agent_route(req: &mut Request, res: &mut Response) {
    let user_agent = req.headers.get("User-Agent").expect("Expected header 'User-Agent'");
    res.status_code = 200;
    res.status_text = "OK".to_string();
    res.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    res.headers.insert("Content-Length".to_string(), user_agent.len().to_string());
    res.body = user_agent.clone();
}

pub fn make_get_file_route(directory: &String) -> Route {
    Box::new(move |req: &mut Request, res: &mut Response| {
        let filename = req.params.get("filename").expect("Expected parameter 'filename'");

        let file_path: PathBuf = Path::new(&directory).join(filename);

        if !file_path.exists() {
            not_found_route(req, res);
            return;
        }

        match fs::read(&file_path) {
            Ok(contents) => {
                res.status_code = 200;
                res.status_text = "OK".to_string();
                res.headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
                res.headers.insert("Content-Length".to_string(), contents.len().to_string());
                res.body = String::from_utf8_lossy(&contents).into_owned();
            },
            Err(_) => {
                not_found_route(req, res);
            }
        }
    })
}

pub fn make_post_file_route(directory: &String) -> Route {
    Box::new(move |req: &mut Request, res: &mut Response| {
        let filename = req.params.get("filename").expect("Expected parameter 'filename'");

        let file_path = Path::new(&directory).join(filename);

        let content = req.body.clone();

        match fs::write(&file_path, content) {
            Ok(_) => {
                res.status_code = 201;
                res.body = "Created".to_string();
            },
            Err(e) => {
                make_interval_server_error_route(e)(req, res);
            }
        }
    })
}
