use crate::router::Router;
use crate::routes::{echo_route, make_get_file_route, make_post_file_route, root_route, user_agent_route};
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
    let directory = directory.as_deref().unwrap_or("/tmp/").to_string();

    let mut router = Router::new();
    // Register routes with their methods.
    router.register("GET", "/", Box::new(root_route));
    router.register("GET", "/echo/{msg}", Box::new(echo_route));
    router.register("GET", "/user-agent", Box::new(user_agent_route));
    router.register(
        "GET",
        "/files/{filename}",
        make_get_file_route(&directory),
    );
    router.register(
        "GET",
        "/files/{filename}",
        make_post_file_route(&directory),
    );

    server::run(ip, port, router)
}
