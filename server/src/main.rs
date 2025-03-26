mod middlewares;
mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("Database URL required: DATABASE_URL environment variable must be set");
    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await?;
    println!("{:?}", pg_pool);
    let api = axum::Router::new()
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth_middleware::authenticate,
        ))
        .nest("/auth", routes::auth_routes::auth_router());
    Ok(())
}
