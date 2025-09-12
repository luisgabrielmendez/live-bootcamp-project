use axum::{
    http::StatusCode,
    extract::State,
    Json
};
use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::auth::validate_token};
use serde::Deserialize;

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {
    match validate_token(&request.token, state.banned_token_store.clone()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
}
