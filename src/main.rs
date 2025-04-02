use std::io::{BufRead, Write};
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                let mut buf_reader = std::io::BufReader::new(&mut stream);

                let request = Request::parse(&mut buf_reader);

                println!("{:?}", request);

                let response = match request.request_line.as_str() {
                    "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\n",
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                };

                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

#[derive(Debug)]
struct Request {
    request_line: String,
    headers: String,
    body: String,
}

impl Request {
    fn parse<R: BufRead>(reader: &mut R) -> Self {
        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let mut headers = String::new();
        let mut header_line = String::new();
        while reader.read_line(&mut header_line).unwrap() > 0 {
            if header_line.trim().is_empty() {
                break;
            }
            headers.push_str(&header_line);
            header_line.clear();
        }

        let mut body = String::new();
        reader.read_to_string(&mut body).unwrap();

        Request {
            request_line: request_line.trim().to_string(),
            headers,
            body,
        }
    }
}
