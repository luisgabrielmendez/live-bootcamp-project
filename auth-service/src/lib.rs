use std::error::Error;

use axum::{
    http::Method,
    routing::post,
    serve::Serve,
    Router};

use app_state::AppState;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};

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
    //                                //  This function builds the axum server.
    //                                //  It can be used in main (production) or in tests (development).
    //                                //  The same code is used for both.
    //                                //  All tests run an instance of the application the same way it wil be
    //                                //  done in production.
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        //                            //  Allow the app service(running on our local machine and in
        //                            //  production) to call the auth service.

        //                            //  CORS (Cross-Origin Resource Sharing)
        //                            //  Setup allowed ip's and ports.
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

        //                            //  AXUM APPLICATION
        //                            //  Setup ROUTES
        //                            //  In most of the videos I've seen these routes are in main.
        //                            //  Moving routes from `main,rs' to here enables to call it from different
        //                            //  sources: main and test.
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors);
        //                            //  Setup LISTENER
        //                            //  Binds to address received as parameter
        let listener = tokio::net::TcpListener::bind(address).await?;
        //                            //  Extract the address from listener.
        //                            //  Checks the listener is working fine.
        let address = listener.local_addr()?.to_string();
        //                            //  Setup SERVER
        let server = axum::serve(listener, router);

        //                            //  Create a new Application instance and return it as Ok variant.
        //                            //  With the builded server and the extracted address from listener.
        Ok(Application {
            server : server,
            address: address
        })
    }

    //                                //  Runs the application
    pub async fn run(self, origin: String) -> Result<(), std::io::Error> {
        println!("listening on {} for {}", &self.address, origin);
        self.server.await
    }
}

