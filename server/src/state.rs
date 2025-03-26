#[derive(Clone)]
pub struct State {
    pub pg_pool: sqlx::postgres::PgPool,
    pub jwt_key_provider: crate::jwt::JWTKeyProvider,
}

impl State {
    pub fn new(pool: sqlx::postgres::PgPool, jwt_secret: &str) -> Self {
        State {
            pg_pool: pool,
            jwt_key_provider: crate::jwt::JWTKeyProvider::new(jwt_secret),
        }
    }
}
