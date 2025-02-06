mod account;

use actix_web::web::{self, ServiceConfig};
use actix_web::{HttpResponse, get};
use serde_json::json;

#[get("/")]
async fn status() -> HttpResponse {
    HttpResponse::Ok().json(json! {{ "status": "OK" }})
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(status).service(
        web::scope("/api")
            .service(account::signup)
            .service(account::login)
            .service(account::logout)
            .service(account::delete_account)
            .service(account::delete_session),
    );
}
