use crate::app_state::AppState;
use crate::models::user::User;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Json;

#[derive(serde::Deserialize)]
struct SignupPayload {
    email: String,
    password: String,
}

#[derive(serde::Serialize)]
struct SignupResponse {
    message: String,
}

#[derive(serde::Serialize)]
struct ErrorMessage {
    message: String,
}

#[axum::debug_handler]
async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupPayload>,
) -> Result<Json<SignupResponse>, (StatusCode, Json<ErrorMessage>)> {
    let user = match User::get_user_by_email(state.pg_pool.clone(), &payload.email).await {
        Ok(user) => Some(user),
        Err(_) => None,
    };

    if user.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorMessage {
                message: "Email already exists".to_string(),
            }),
        ));
    }

    let id = match User::create_user_with_email_and_password(
        state.pg_pool,
        &payload.email,
        payload.password,
    )
    .await
    {
        Ok(id) => id,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorMessage {
                    message: err.to_string(),
                }),
            ))
        }
    };

    Ok(Json(SignupResponse {
        message: format!("Account {} created successfully", id),
    }))
}

#[derive(serde::Deserialize)]
struct SigninPayload {
    email: String,
    password: String,
}

#[derive(serde::Serialize)]
struct SigninResponse {
    token: String,
}

async fn signin(
    State(state): State<AppState>,
    Json(payload): Json<SigninPayload>,
) -> Result<Json<SigninResponse>, (StatusCode, Json<ErrorMessage>)> {
    let user = match User::get_user_by_email(state.pg_pool, &payload.email).await {
        Ok(user) => user,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorMessage {
                    message: err.to_string(),
                }),
            ))
        }
    };
    if password_auth::verify_password(payload.password, &user.password).is_ok() {
        use jwt_simple::prelude::*;
        let mut claims = Claims::create(Duration::from_hours(2));
        claims.subject = Some(user.id.to_string());
        let token = state
            .jwt_key_provider
            .key_pair
            .sign(claims)
            .unwrap_or(String::new());

        Ok(Json(SigninResponse { token }))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorMessage {
                message: "Invalid email or password".to_string(),
            }),
        ))
    }
}

pub fn auth_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
    //.route("/signout", post(signout))
}
