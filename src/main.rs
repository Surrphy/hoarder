use axum::{
    routing::get,
    Router,
};

use sqlx::postgres::PgPoolOptions;
use tokio::sync::Mutex;

use std::collections::HashMap;
use std::sync::Arc;
use std::env;

use hoarder::routes;
use hoarder::utils::AppState;

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not set");
    let url = env::var("APP_URL").expect("APP_URL env var not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Couldn't connect to DB");

    let shared_state = Arc::new(Mutex::new(AppState { pool, connected_users: HashMap::new() }));

    sqlx::migrate!("db/migrations")
        .run(&shared_state.lock().await.pool)
        .await
        .expect("Coudn't run migrations on database");

    // build our application with a single route let app = Router::new().route("/", get(json_response));
    let app = Router::new().route("/login", get(routes::users::login)).with_state(Arc::clone(&shared_state));

    // run it with hyper on addr given in envvar
    axum::Server::bind(&url.parse().expect(&format!("Couldn't bind to address: {}", url)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
