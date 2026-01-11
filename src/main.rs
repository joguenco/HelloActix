mod ping;
use ping::ping as ping_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(ping_handler))
        .bind(("127.0.0.1", 8080))?
        .workers(3)
        .run()
        .await
}
