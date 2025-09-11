use std::error::Error;

use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router};
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};

use app_state::AppState;
use tower_http::{cors::CorsLayer, services::ServeDir};

use routes::*;

pub mod routes;
pub mod app_state;
pub mod services;
pub mod domain;
pub mod utils;

//                                    //  This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    //                                //  address is exposed as a public field
    //                                //  so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        //                            //  Allow the app service(running on our local machine and in
        //                            //  production) to call the auth service.
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            //                        //  DONE-TODO:
            //                        //  Replace [YOUR_DROPLET_IP] with your Droplet IP address
            "http://159.223.97.107:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            //                        //  Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            //                        //  Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        //                            //  Move the Router definition from `main.rs` to here.
        //                            //  Also, remove the `hello` route.
        //                            //  We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        //                            //  Create a new Application instance and return it
        Ok(Application {
            server : server,
            address: address
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),            
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}