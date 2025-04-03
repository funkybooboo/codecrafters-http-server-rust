use crate::router::Router;
use crate::routes::{echo_route, make_file_route, root_route, user_agent_route};
use std::env;

mod server;
mod request;
mod router;
mod routes;
mod response;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let directory = args
        .windows(2)
        .find(|pair| pair[0] == "--directory")
        .map(|pair| pair[1].clone());

    if let Some(ref dir) = directory {
        println!("Directory provided: {}", dir);
    } else {
        println!("No directory provided.");
    }

    let ip = "127.0.0.1";
    let port = 4221;

    let mut router = Router::new();
    // Box each route to match the expected `Route` type.
    router.register("/", Box::new(root_route));
    router.register("/echo/{msg}", Box::new(echo_route));
    router.register("/user-agent", Box::new(user_agent_route));
    router.register("/files/{filename}", Box::new(make_file_route(
        directory.as_deref().unwrap_or("/tmp/").to_string()
    )));

    server::run(ip, port, router)
}
