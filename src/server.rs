use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use crate::request::Request;
use crate::response::Response;
use crate::router::Router;
use flate2::write::GzEncoder;
use flate2::Compression;

pub fn run(ip: &str, port: u16, router: Router) -> std::io::Result<()> {
    let address = format!("{}:{}", ip, port);
    println!("Server started on http://{}", address);

    let listener = TcpListener::bind(address)?;
    // Wrap the router in an Arc so it can be shared safely among threads.
    let router = Arc::new(router);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted new connection");
                let router = Arc::clone(&router);
                std::thread::spawn(move || {
                    if let Err(e) = handle_connection(stream, &router) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, router: &Router) -> std::io::Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut request = Request::parse(&mut reader)?;
    println!("Request: {:?}", request);

    let mut response: Response = router.route(&mut request);

    if let Some(accept_encoding) = request.headers.get("Accept-Encoding") {
        if accept_encoding
            .split(',')
            .map(|s| s.trim())
            .any(|enc| enc == "gzip")
        {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all((&response.body).as_ref())?;
            let compressed_body = encoder.finish()?;

            response.headers.insert("Content-Encoding".to_string(), "gzip".to_string());
            response.headers.insert("Content-Length".to_string(), compressed_body.len().to_string());
            response.body = compressed_body;
        }
    }

    stream.write_all((&response.format_response()).as_ref())?;
    Ok(())
}
