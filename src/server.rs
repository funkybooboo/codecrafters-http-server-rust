use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use crate::request::Request;
use crate::handlers::{Response, route_request};

/// Runs the TCP server on the specified IP and port.
pub fn run(ip: &str, port: u16) -> std::io::Result<()> {
    let address = format!("{}:{}", ip, port);
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
