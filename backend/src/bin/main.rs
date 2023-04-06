use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8080))?;
    pace::run(listener)?.await
}
