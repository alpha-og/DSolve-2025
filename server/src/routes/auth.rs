use axum::routing::post;

use crate::state::State;

pub fn auth_router() -> axum::Router<State> {
    axum::Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
        .route("/signout", post(signout))
}
