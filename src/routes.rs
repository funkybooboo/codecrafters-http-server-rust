use crate::request::Request;
use crate::response::Response;

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
