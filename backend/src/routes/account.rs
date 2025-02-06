use actix_web::{HttpResponse, post, web::Json};

#[derive(Debug, Clone, Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

#[post("/acc/signup")]
pub async fn signup(req: Json<SignupRequest>) -> HttpResponse {
    let req = req.into_inner();
    
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
