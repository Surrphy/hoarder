use axum::{
    routing::get,
    Router,
};

use tokio::sync::Mutex;

use sqlx::PgPool;

use std::collections::HashMap;
use std::sync::Arc;

use hoarder::routes;
use hoarder::utils::AppState;

#[shuttle_runtime::main]
pub async fn axum(
#[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {

    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    let shared_state = Arc::new(Mutex::new(AppState { pool, connected_users: HashMap::new() }));

    // build our application with a single route let app = Router::new().route("/", get(json_response));
    let app = Router::new()
        .route("/login", get(routes::users::login)).with_state(Arc::clone(&shared_state))
        .route("/secret", get(routes::users::get_secret)).with_state(Arc::clone(&shared_state));

    Ok(app.into())
}
