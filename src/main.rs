use crate::server::server;

mod apis;
mod configurations;
mod probes;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
