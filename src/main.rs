use axum::{
    routing::get,
    extract::State,
    Router,
};

use sqlx::postgres::PgPoolOptions;

use std::sync::Arc;
use std::env;

use hoarder::{routes, AppState};

#[tokio::main]
async fn main() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not set");
    let url = env::var("APP_URL").expect("APP_URL env var not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Couldn't connect to DB");

    let shared_state = Arc::new(AppState { pool });

    // build our application with a single route let app = Router::new().route("/", get(json_response));
    let app = Router::new().route("/get_users", get(routes::get_users)).with_state(shared_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&url.parse().expect(&format!("Couldn't bind to address: {}", url)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
