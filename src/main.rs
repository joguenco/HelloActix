mod ping;
mod version;
use crate::ping::mod_ping::ping as ping_handler;
use crate::version::mod_version::version as version_handler;
use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use std::env;

use actix_web::{App, HttpServer, middleware::Logger, web};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let con = connect();

    let pool_data = web::Data::new(con);

    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .wrap(Logger::default())
            .service(ping_handler)
            .service(version_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(3)
    .run()
    .await
}

pub fn connect() -> DBPool {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("faild to get DB Url");
    let db = ConnectionManager::new(db_url);

    r2d2::Pool::builder()
        .max_size(5)
        .build(db)
        .expect("faild to create db pool")
}
