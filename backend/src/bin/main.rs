use pace::get_connection_pool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8080))?;
    let db_pool = get_connection_pool(None).expect("Could not initialize connection pool");
    pace::run(listener, db_pool, None)?.await
}
