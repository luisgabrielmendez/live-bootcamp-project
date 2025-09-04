use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState, 
    domain::{AuthAPIError, Email, Password, User}
};

pub async fn signup(
    //                                //  DONE-TODO:
    //                                //  Use Axum's state extractor to pass in AppState
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    //                                //  DONE-TODO:
    //                                //  Early return AuthAPIError::InvalidCredentials if:
    //                                //  - email is empty or does not contain '@'
    //                                //  - password is less than 8 characters

    //                                //  Get email value from request
    // let email = request.email;
    //                                //  Validate email
    // if email == "" || !email.contains("@") {
    //     return Err(AuthAPIError::InvalidCredentials);
    // }
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;    

    //                                //  Get password value from request
    // let password = request.password;
    //                                //  Validate password
    // if password.len() < 8 {
    //     return Err(AuthAPIError::InvalidCredentials);
    // }
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;



    //                                //  DONE-TODO:
    //                                //  Create a new `User` instance using data in the request
    let user = User::new(email,
                               password,
                               request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    //                                //  DONE-TODO:
    //                                //  Early return AuthAPIError::UserAlreadyExists if email exists in
    //                                //  user_store.
    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }
    
    //                                //  DONE-TODO:
    //                                //  Add `user` to the `user_store`.
    //                                //  Simply unwrap the returned `Result` enum type for now.

    //                                //  DONE-TODO:
    //                                //  instead of using unwrap, early return AuthAPIError::UnexpectedError
    //                                //  if add_user() fails.
    // user_store.add_user(user).unwrap();
    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize, Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}