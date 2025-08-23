use crate::helpers::{self, TestApp};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

// TODO: Implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token)
// For now, simply assert that each route returns a 200 HTTP status code.

#[tokio::test]
async fn signup_registers_new_user() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn authenticate_user_and_return_jwt() {
    let app = TestApp::new().await;

    let response = app.post_login().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn verify_2fa_token() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn logout_user() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn verify_jwt() {
    let app = TestApp::new().await;

    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
