#[derive(Clone)]
pub struct AppState {
    pub pg_pool: sqlx::postgres::PgPool,
    pub jwt_key_provider: crate::jwt::JWTKeyProvider,
}

impl AppState {
    pub fn new(pool: sqlx::postgres::PgPool, jwt_secret: &str) -> Self {
        Self {
            pg_pool: pool,
            jwt_key_provider: crate::jwt::JWTKeyProvider::new(jwt_secret),
        }
    }
}
