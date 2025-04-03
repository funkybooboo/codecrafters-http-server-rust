use crate::router::Router;
use crate::routes::{echo_route, make_file_route, root_route, user_agent_route};
use std::env;

mod server;
mod request;
mod router;
mod routes;
mod response;

fn main() -> std::io::Result<()> {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Look for "--directory" followed by a value using the `windows` iterator.
    let directory = args
        .windows(2)
        .find(|pair| pair[0] == "--directory")
        .map(|pair| pair[1].clone());

    // Log the directory or indicate that none was provided.
    if let Some(ref dir) = directory {
        println!("Directory provided: {}", dir);
    } else {
        println!("No directory provided.");
    }

    let ip = "127.0.0.1";
    let port = 4221;

    let mut router = Router::new();
    router.register("/", root_route);
    router.register("/echo/{msg}", echo_route);
    router.register("/user-agent", user_agent_route);
    router.register("/files/{filename}", make_file_route(
        directory.as_deref().unwrap_or("/tmp/").parse().unwrap()
    ));

    server::run(ip, port, router)
}
