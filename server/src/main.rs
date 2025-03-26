mod app_state;
mod jwt;
mod middlewares;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load jwt_secret
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT secret key required: JWT_SECRET environment variable must be set");

    // setup database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("Database URL required: DATABASE_URL environment variable must be set");
    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await?;

    // initialize server state
    let state = app_state::AppState::new(pg_pool, &jwt_secret);
    let api = axum::Router::new()
        .route("/protected", axum::routing::get(async || "protected"))
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth::authenticate,
        ))
        .nest("/auth", routes::auth::auth_router());

    let app = axum::Router::new()
        .route("/", axum::routing::get(async || "ok"))
        .nest("/api", api)
        .layer(
            tower_http::cors::CorsLayer::new().allow_origin(
                "http://localhost:5173"
                    .parse::<axum::http::HeaderValue>()
                    .expect("Failed to parse origin"),
            ),
        ) // tower_http service layer for CORS
        .with_state(state); // supply all routes with application state
                            /* create TCP listener */
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("server listening on {}", listener.local_addr().unwrap());

    /* start the axum server */
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
