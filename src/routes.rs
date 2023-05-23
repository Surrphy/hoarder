pub mod users {
    use serde::{Deserialize, Serialize};
    
    use axum::response::Json;
    use axum::extract::{State, Path};
    use axum::http::StatusCode;
    
    use std::sync::Arc;
    
    use crate::app::AppState;

    #[derive(Deserialize, Serialize)]
    pub struct User {
        user_id: i32,
        name: Option<String>,
        surname: Option<String>
    }

    #[derive(Deserialize, Serialize)]
    pub struct UserQuery {
        user_id: i32
    }
    
    pub async fn get_users(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, StatusCode> {
        let results = match sqlx::query_as!(User, "SELECT user_id, name, surname FROM users").fetch_all(&state.pool).await {
            Ok(val) => val,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
        };
    
        Ok(Json(results))
    }
    
    pub async fn get_user(Path(params): Path<UserQuery>,State(state): State<Arc<AppState>>) -> Result<Json<User>, StatusCode> {
        let result = match sqlx::query_as!(User, "SELECT user_id, name, surname FROM users WHERE user_id = $1", params.user_id).fetch_one(&state.pool).await {
            Ok(val) => val,
            Err(sqlx::Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
        };
        
        Ok(Json(result))
    }
}
