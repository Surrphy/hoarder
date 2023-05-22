use sqlx::postgres::PgPool;

pub mod routes;

pub struct AppState {
    pub pool: PgPool
}

#[cfg(test)]
mod tests {
    use super::*;
    
    //#[tokio::test]
    //async fn 
}
