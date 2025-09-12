use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};

use serde::{Deserialize, Serialize};

//                                    //  An enum to define valid Auth API Errors
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
    UnexpectedError,
}

//                                    //  This implementation was in lib.rs, as Bogdan did it.
impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),            
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

//                                    //  This is the error response used in all errors.
//                                    //  I don't like it in here 
//                                    //  FUTURE: Extract from here and put it in src/domain/error.rs.
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
