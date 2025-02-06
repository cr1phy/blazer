use actix_web::{HttpResponse, post};

#[post("/acc/signup")]
pub async fn signup() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/acc/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/acc/logout")]
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/acc/delete")]
pub async fn delete_account() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/acc/session/delete")]
pub async fn delete_session() -> HttpResponse {
    HttpResponse::Ok().finish()
}
