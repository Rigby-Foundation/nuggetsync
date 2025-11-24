use redis::{Client, Commands, RedisResult};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionData {
    pub user_id: i32,
    pub ip: String,
}

pub fn get_redis_client() -> RedisResult<Client> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    Client::open(redis_url)
}

pub fn create_session(client: &mut Client, user_id: i32, ip: String) -> Result<String, String> {
    let token = Uuid::new_v4().to_string();
    let session_key = format!("session:{}", token);
    let session_data = SessionData { user_id, ip };
    let session_json = serde_json::to_string(&session_data).map_err(|e| e.to_string())?;

    let _: () = client
        .set_ex(session_key, session_json, 24 * 60 * 60)
        .map_err(|e| e.to_string())?;

    Ok(token)
}

pub fn validate_session(client: &mut Client, token: &str, current_ip: &str) -> Result<i32, String> {
    let session_key = format!("session:{}", token);
    let session_json: String = client
        .get(&session_key)
        .map_err(|_| "Session not found".to_string())?;

    let session_data: SessionData =
        serde_json::from_str(&session_json).map_err(|e| e.to_string())?;

    if session_data.ip != current_ip {
        let _: () = client.del(&session_key).unwrap_or(());
        return Err("IP address changed".to_string());
    }

    Ok(session_data.user_id)
}
