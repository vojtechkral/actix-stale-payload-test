use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    actix_stale_payload::server(8080).await
}
