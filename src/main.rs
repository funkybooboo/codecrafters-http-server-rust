use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

type Response = String;

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:4221";
    println!("Server started on http://{}", address);

    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted new connection");
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling connection: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let request = Request::parse(&mut reader)?;
    println!("Request: {:?}", request);

    let response: Response = route_request(&request);
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn route_request(request: &Request) -> Response {
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

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    http_version: String,
    headers: Vec<String>,
    body: String,
}

impl Request {
    fn parse<R: BufRead>(reader: &mut R) -> std::io::Result<Self> {
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;
        let request_line = request_line.trim_end();
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let http_version = parts.next().unwrap_or("").to_string();

        let headers = Self::parse_headers(reader)?;
        // For now, we're not handling the body.
        let body = String::new();

        Ok(Request {
            method,
            path,
            http_version,
            headers,
            body,
        })
    }

    fn parse_headers<R: BufRead>(reader: &mut R) -> std::io::Result<Vec<String>> {
        let mut headers = Vec::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 || line.trim().is_empty() {
                break;
            }
            headers.push(line.trim_end().to_string());
        }
        Ok(headers)
    }
}
