use crate::schema::profiles;
use crate::usermodule::AuthUser;
use crate::utils::db::DbPool;
use axum::{Json, extract::State, http::StatusCode};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub hash: String,
    pub name: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = profiles)]
pub struct NewProfile<'a> {
    pub user_id: i32,
    pub hash: &'a str,
    pub name: &'a str,
}

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    pub name: String,
    pub hash: String,
    pub encryption_type: String,
}

pub async fn create_profile(
    State(pool): State<DbPool>,
    auth_user: AuthUser,
    Json(payload): Json<CreateProfileRequest>,
) -> Result<Json<Profile>, (StatusCode, String)> {
    if payload.encryption_type != "XChaCha20-Poly1305" {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid encryption type. Only XChaCha20-Poly1305 is supported.".to_string(),
        ));
    }

    let mut conn = pool
        .get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_profile = NewProfile {
        user_id: auth_user.user_id,
        hash: &payload.hash,
        name: &payload.name,
    };

    let profile = diesel::insert_into(profiles::table)
        .values(&new_profile)
        .get_result::<Profile>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(profile))
}

pub async fn get_profiles(
    State(pool): State<DbPool>,
    auth_user: AuthUser,
) -> Result<Json<Vec<Profile>>, (StatusCode, String)> {
    let mut conn = pool
        .get()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    use crate::schema::profiles::dsl::{profiles as profiles_table, user_id};
    let results = profiles_table
        .filter(user_id.eq(auth_user.user_id))
        .load::<Profile>(&mut conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(results))
}
