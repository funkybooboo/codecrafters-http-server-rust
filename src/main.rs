use crate::router::Router;
use crate::routes::{echo_route, root_route, user_agent_route};

mod server;
mod request;
mod router;
mod routes;
mod response;

fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 4221;

    let mut router = Router::new();
    router.register("/", root_route);
    router.register("/echo/{msg}", echo_route);
    router.register("/user-agent", user_agent_route);

    server::run(ip, port, router)
}
