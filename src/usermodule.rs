use crate::schema::users;
use crate::utils::db::DbPool;
use crate::utils::redis::{create_session, get_redis_client};
use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use axum::{
    Json, RequestPartsExt,
    extract::{ConnectInfo, FromRequestParts, State},
    http::{StatusCode, request::Parts},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String,
}

pub async fn register_user(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool
        .get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    use crate::schema::users::dsl::{username, users as users_table};
    let user_exists = users_table
        .filter(username.eq(&payload.username))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if user_exists.is_some() {
        return Err((StatusCode::CONFLICT, "Username already exists".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();

    let new_user = NewUser {
        username: &payload.username,
        password: &password_hash,
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

pub async fn login_user(
    State(pool): State<DbPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<LoginUserRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let mut conn = pool
        .get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    use crate::schema::users::dsl::{username, users as users_table};
    let user = users_table
        .filter(username.eq(&payload.username))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        ))?;

    let argon2 = Argon2::default();
    let parsed_hash = argon2::PasswordHash::new(&user.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        ));
    }

    let mut redis_client =
        get_redis_client().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let token = create_session(&mut redis_client, user.id, addr.ip().to_string())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(LoginResponse { token, user }))
}

pub struct AuthUser {
    pub user_id: i32,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = {
            let auth_header = parts
                .headers
                .get(axum::http::header::AUTHORIZATION)
                .ok_or((
                    StatusCode::UNAUTHORIZED,
                    "Missing Authorization header".to_string(),
                ))?
                .to_str()
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Invalid Authorization header".to_string(),
                    )
                })?;

            if !auth_header.starts_with("Bearer ") {
                return Err((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()));
            }
            auth_header[7..].to_string()
        };

        let ConnectInfo(addr) = parts
            .extract::<ConnectInfo<SocketAddr>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Could not get client IP".to_string(),
                )
            })?;

        let mut redis_client =
            get_redis_client().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let user_id = crate::utils::redis::validate_session(
            &mut redis_client,
            &token,
            &addr.ip().to_string(),
        )
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

        Ok(AuthUser { user_id })
    }
}
