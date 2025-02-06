use crate::entity::prelude::User;
use crate::entity::user;
use crate::errors::ServerError;
use crate::state::AppState;
use actix_web::{post, web::Json, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct SignupResponse {
    token: String,
}

#[post("/acc/signup")]
pub async fn signup(req: Json<SignupRequest>, state: AppState) -> Result<HttpResponse, ServerError> {
    let db = &state.db;
    let req = req.into_inner();
    let hashed_password = hash(&req.password, DEFAULT_COST).map_err(|_| ServerError::InternalError)?;

    let is_existed = User::find().filter(user::Column::Username.eq(&req.username).or(user::Column::Email::eq(&req.email))).one(db).await.map_err(|_| ServerError::InternalError)?.is_some();
    if is_existed {
        return Err(ServerError::UserFound);
    }

    let user_model = user::ActiveModel {
        email: Set(req.email),
        username: Set(req.username),
        password: Set(hashed_password.into_bytes()),
        created_at: Set(Utc::now().naive_utc()),
        last_online: Set(Utc::now().naive_utc()),
        ..Default::default()
    }.insert(db).await.map_err(|_| ServerError::InternalError);

    Ok(HttpResponse::Ok().json(SignupResponse { token: String::new() }))
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
