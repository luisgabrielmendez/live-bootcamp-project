use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState,
    services::{
        hashmap_user_store::HashmapUserStore,
        hashset_banned_token_store::HashsetBannedTokenStore},
    utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {
    //                                //  DONE-TODO:
    //                                //  Create new instance of user_store.
    //                                //  This user store simulates the data base.
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    //                                //  DONE-TODO:
    
    let banned_token_store = 
        Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    //                                //  Create new instance of app_state.
    let app_state = AppState::new(user_store, banned_token_store);

    //                                //  Builds the application
    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    //                                //  Runs the Application -> Axum Server
    app.run("main".to_owned()).await.expect("Failed to run app");
}

