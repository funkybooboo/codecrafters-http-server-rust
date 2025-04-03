use crate::request::Request;
use crate::response::Response;
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

pub fn make_file_route(directory: String) -> impl Fn(&mut Request, &mut Response) {
    move |req: &mut Request, res: &mut Response| {
        let filename = req.params.get("filename").expect("Expected parameter 'filename'");

        // Build the full file path.
        let file_path: PathBuf = Path::new(&directory).join(filename);

        // Check if the file exists.
        if !file_path.exists() {
            not_found_route(req, res);
            return;
        }

        // Attempt to read the file contents.
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
    }
}
