use crate::entity::prelude::User;
use crate::entity::user::{self, Column as UserColumn};
use crate::errors::ServerError;
use crate::state::AppState;
use crate::utils::jwt::generate_token;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

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
pub async fn signup(
    req: Json<SignupRequest>,
    state: Data<AppState>,
) -> Result<HttpResponse, ServerError> {
    let db = &state.db;
    let req = req.into_inner();
    let hashed_password =
        hash(&req.password, DEFAULT_COST).map_err(|_| ServerError::InternalError)?;

    let is_existed = User::find()
        .filter(
            UserColumn::Username
                .eq(&req.username)
                .and(UserColumn::Email.eq(&req.email)),
        )
        .one(db)
        .await
        .map_err(|_| ServerError::InternalError)?
        .is_some();
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
    }
    .insert(db)
    .await
    .map_err(|_| ServerError::InternalError)?;

    let token = generate_token(user_model.id, "21211223ed".into());

    Ok(HttpResponse::Ok().json(SignupResponse {
        token: String::new(),
    }))
}

#[derive(Debug, Clone, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    session_id: String,
}

#[post("/acc/login")]
pub async fn login(
    req: Json<LoginRequest>,
    state: Data<AppState>,
) -> Result<HttpResponse, ServerError> {
    let db = &state.db;
    let req = req.into_inner();

    let user = User::find()
        .filter(user::Column::Email.eq(&req.email))
        .one(db)
        .await
        .map_err(|_| ServerError::InternalError)?;

    if user.is_none() {
        return Err(ServerError::UserNotFound);
    }
    let user = user.unwrap();

    if verify(
        &req.password,
        String::from_utf8(user.password.clone()).unwrap().as_str(),
    )
    .map_err(|_| ServerError::InternalError)?
    {
        return Err(ServerError::UserNotFound);
    }

    Ok(HttpResponse::Ok().json(LoginResponse {
        token: "".to_string(),
        session_id: "".to_string(),
    }))
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
