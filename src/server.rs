use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use crate::request::Request;
use crate::response::Response;
use crate::router::Router;

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

    let response: Response = router.route(&mut request);
    stream.write_all(response.format_response().as_bytes())?;
    Ok(())
}
