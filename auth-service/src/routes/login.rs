use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password}
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    //                                //  Validate request content
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials)
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials)
    }

    let _user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}