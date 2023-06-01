use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use serde::Deserialize;

use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::Mutex;

use std::sync::Arc;

use sqlx::Row;

use crate::utils::{AppState, self};

#[derive(Deserialize)]
pub struct LoginQuery {
    user_fingerprint: String,
}

#[axum_macros::debug_handler]
pub async fn login(State(state): State<Arc<Mutex<AppState>>>, Json(params): Json<LoginQuery>) -> Result<String, StatusCode> {
    let mut state = state.lock().await;
    let user = utils::get_user(&state.pool, &params.user_fingerprint).await?;

    let api_token: String = rand::thread_rng().sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let secret = utils::encrypt(&api_token, user.get_pub_key()).await?;

    state.add_connected_user(user, api_token);

    Ok(secret)
}

#[derive(Deserialize)]
pub struct RegisterQuery{
    user_fingerprint: String,
    user_pub_key: String,
}

pub async fn register(State(state): State<Arc<Mutex<AppState>>>, Json(params): Json<RegisterQuery>) -> Result<String, StatusCode> {
    let state = state.lock().await;

    if let Ok(_) = utils::get_user(&state.pool, &params.user_fingerprint).await {
        return Err(StatusCode::BAD_REQUEST);
    }

    let Ok(_) = sqlx::query("
INSERT INTO 
users (user_fingerprint, user_public_key, is_admin)
VALUES ($1, $2, false)")
        .bind(params.user_fingerprint)
        .bind(params.user_pub_key)
        .execute(&state.pool)
        .await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    Ok(String::from("OK"))
}

#[derive(Deserialize)]
pub struct Request {
    user_fingerprint: String,
    api_token: String,
}

#[axum_macros::debug_handler]
pub async fn get_secret(State(state): State<Arc<Mutex<AppState>>>, Json(params): Json<Request>) -> Result<String, StatusCode> {
    let (pub_key, api_token) = match state.lock().await.connected_users.get(&params.user_fingerprint) {
        Some(val) => (val.0.get_pub_key().clone(), val.1.to_string()),
        None => return Err(StatusCode::UNAUTHORIZED)
    };

    if params.api_token != api_token {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    Ok(utils::encrypt("Sekretna wiadomość Huberta Moszki", &pub_key).await?) 
}
