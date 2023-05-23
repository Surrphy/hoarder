use sqlx::postgres::PgPool;

pub struct AppState {
    pub pool: PgPool
}
