mod server;
mod request;
mod handlers;

fn main() -> std::io::Result<()> {
    server::run()
}
