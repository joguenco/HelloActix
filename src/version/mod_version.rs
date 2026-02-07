use crate::DBPool;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder, get};
use serde::Serialize;

#[derive(Serialize)]
struct VersionResponse {
    version: String,
}

#[get("/version")]
pub async fn version(db: Data<DBPool>) -> impl Responder {
    match db.get() {
        Ok(mut conn) => {
            let p = VersionResponse {
                version: "1.0.0".to_string(),
            };
            HttpResponse::Ok().json(p)
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
