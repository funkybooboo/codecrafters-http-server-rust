use std::io::{BufRead, Write};
use std::net::TcpListener;

fn main() {
    println!("Server started on http://127.0.0.1:4221");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let mut buf_reader = std::io::BufReader::new(&mut stream);
                let request = Request::parse(&mut buf_reader);

                println!("{:?}", request);

                let response = endpoint_dispatcher(&request);

                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn endpoint_dispatcher(req: &Request) -> String {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => handle_root(),
        ("GET", path) if path.starts_with("/echo/") => handle_echo(path),
        _ => handle_404(),
    }
}

fn handle_root() -> String {
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

fn handle_echo(path: &str) -> String {
    let echoed_string = path.strip_prefix("/echo/").unwrap_or("");
    let content_length = echoed_string.len();

    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content_length, echoed_string
    )
}

fn handle_404() -> String {
    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    http_version: String,
    headers: String,
    body: String,
}

impl Request {
    fn parse<R: BufRead>(reader: &mut R) -> Self {
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();
        let request_line = request_line.trim();

        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let http_version = parts.next().unwrap_or("").to_string();

        let mut headers = String::new();
        let mut header_line = String::new();
        while reader.read_line(&mut header_line).unwrap() > 0 {
            if header_line.trim().is_empty() {
                break;
            }
            headers.push_str(&header_line);
            header_line.clear();
        }

        let body = String::from(""); // For now, keep this empty

        Request {
            method,
            path,
            http_version,
            headers,
            body,
        }
    }
}
