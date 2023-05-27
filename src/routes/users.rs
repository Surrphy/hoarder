use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use serde::Deserialize;

use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::Mutex;

use std::sync::Arc;

use crate::utils::{AppState, self};

#[derive(Deserialize)]
pub struct LoginQuery {
    user_fingerprint: String,
}

#[axum_macros::debug_handler]
pub async fn login(State(state): State<Arc<Mutex<AppState>>>, Json(params): Json<LoginQuery>) -> Result<String, StatusCode> {
    let mut state = state.lock().await;
    let user = match utils::get_user(&state.pool, &params.user_fingerprint).await {
        Ok(val) => val,
        Err(err) => return Err(err)
    };

    let api_token: String = rand::thread_rng().sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let secret = utils::encrypt(&api_token, user.get_pub_key()).await?;

    state.add_connected_user(user, api_token);

    Ok(secret)
}

#[derive(Deserialize)]
pub struct Request {
    user_fingerprint: String,
    api_token: String,
}

#[axum_macros::debug_handler]
pub async fn get_secret(State(state): State<Arc<Mutex<AppState>>>, Json(params): Json<Request>) -> Result<String, StatusCode> {
    let user = match state.lock().await.connected_users.get(&params.user_fingerprint) {
        Some(val) => val,
        None => return Err(StatusCode::UNAUTHORIZED)
    };
    
    Ok(utils::encrypt("Sekretna wiadomość Huberta Moszki", user.0.get_pub_key()).await?) 
}
