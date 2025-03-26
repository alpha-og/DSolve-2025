use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
};
use jwt_simple::claims::NoCustomClaims;
use jwt_simple::prelude::ECDSAP256PublicKeyLike;

pub async fn authenticate(
    axum::extract::State(state): axum::extract::State<crate::app_state::AppState>,
    headers: HeaderMap,
    mut req: Request<Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let token = match headers.get("authorization") {
        Some(token) => token.to_str().unwrap(),
        None => return Err(StatusCode::UNAUTHORIZED),
    }; //get authorization header from extracted headers
       //return UNAUTHORIZED if authorization header was not found
    let claims = match state
        .jwt_key_provider
        .key_pair
        .public_key()
        .verify_token::<NoCustomClaims>(token, None)
    {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    let id: i32 = claims.subject.unwrap().parse().unwrap();
    req.extensions_mut().insert(id);
    Ok(next.run(req).await)
}
