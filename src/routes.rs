use serde::{Deserialize, Serialize};

use axum::response::Json;
use axum::extract::State;

use anyhow::{Result, anyhow};

use std::sync::Arc;

use super::AppState;

#[derive(Deserialize, Serialize)]
pub struct User {
    user_id: i32,
    name: Option<String>,
    surname: Option<String>
}

pub async fn get_users<T>(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>> {
    let results = match sqlx::query_as!(User, "SELECT user_id, name, surname FROM users").fetch_all(&state.pool).await {
        Ok(val) => val,
        Err(sqlx::Error::RowNotFound) => return Err(anyhow!("Didn't found any users")),
        Err(_) => return Err(anyhow!("Unknown error while querying db"))
    };

    Ok(Json(results))
}
