use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use crate::request::Request;
use crate::response::Response;
use crate::router::Router;

pub fn run(ip: &str, port: u16, router: Router) -> std::io::Result<()> {
    let address = format!("{}:{}", ip, port);
    println!("Server started on http://{}", address);

    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted new connection");
                if let Err(e) = handle_client(stream, &router) {
                    eprintln!("Error handling connection: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, router: &Router) -> std::io::Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut request = Request::parse(&mut reader)?;
    println!("Request: {:?}", request);

    let response: Response = router.route(&mut request);
    stream.write_all(response.format_response().as_bytes())?;
    Ok(())
}
