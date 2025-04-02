mod server;
mod request;
mod router;

fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 4221;
    server::run(ip, port)
}
