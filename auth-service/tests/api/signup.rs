use crate::helpers::TestApp;

#[tokio::test]
async fn signup_registers_new_user() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

