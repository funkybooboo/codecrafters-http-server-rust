use crate::request::Request;
use crate::response::Response;

/// Handler for the root route (static).
pub fn root_route(_req: &mut Request, res: &mut Response) {
    res.status_code = 200;
    res.status_text = "OK".to_string();
}

/// Handler for the echo route (dynamic).
/// Expects the pattern "/echo/{msg}" so that "msg" is extracted into req.params.
pub fn echo_route(req: &mut Request, res: &mut Response) {
    let msg = req.params.get("msg").expect("Expected parameter 'msg'");
    res.status_code = 200;
    res.status_text = "OK".to_string();
    res.headers.insert("Content-Type".to_string(), "text/plain".to_string());
    res.headers.insert("Content-Length".to_string(), msg.len().to_string());
    res.body = msg.clone();
}
