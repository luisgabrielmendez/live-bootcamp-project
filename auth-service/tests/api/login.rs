use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    //                                //  DONE-TODO:
    //                                //  Add 422 test case
    //                                //  To test malformed credentials with login:
    //                                //  1. Create user to login.
    //                                //  2. Create data for test cases.
    //                                //  3. Run test cases

    //                                //  PREPARE TEST ENVIRONMENT
    //                                //  Create app instance
    let app = TestApp::new().await;

    //                                //  1. Create user to login
    //                                //  Create data for test case
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    //                                //  Run test case
    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    //                                //  2. Create data for test cases.
    let test_cases = [
        //                            //  Missing email
        serde_json::json!({
            "password": "password123",
        }),
        //                            //  Missing password
        serde_json::json!({
            "email": random_email,
        }),
        //                            //  Missing email and password
        serde_json::json!({})
    ];

    //                                //  3. Run test cases
    for test_case in test_cases {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }

}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    //                                //  DONE-TODO:
    //                                //  Call the log-in route with invalid credentials and assert that a
    //                                //  400 HTTP status code is returned along with the appropriate error
    //                                //  message.
    
    //                                //  PREPARE TEST ENVIRONMENT
    //                                //  Create app instance
    let app = TestApp::new().await;

    //                                //  1. Create user to login
    //                                //  Create data for test case
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    //                                //  Run test case
    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);
    //                                //  2. Create data for test cases.
    let test_cases = [
        //                            //  Invalid email
        serde_json::json!({
            "email": "invalid_email",
            "password": "password123",
        }),
        //                            //  Invalid password
        serde_json::json!({
            "email": random_email,
            "password": "invalid",
        }),
        //                            //  Missing email
        serde_json::json!({
            "email": "",
            "password": "password123",
        }),
        //                            //  Missing password
        serde_json::json!({
            "email": random_email,
            "password": "",
        }),
        //                            //  Missing email and password
        serde_json::json!({
            "email": "",
            "password": "",
        })
    ];

    for test_case in test_cases {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    //                                //  TODO:
    //                                //  Call the log-in route with incorrect credentials and assert that a
    //                                //  401 HTTP status code is returned along with the appropriate error
    //                                //  message.
    
    //                                //  PREPARE TEST ENVIRONMENT
    //                                //  Create app instance
    let app = TestApp::new().await;

    //                                //  1. Create user to login
    //                                //  Create data for test case
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    //                                //  Run test case
    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);
    //                                //  2. Create data for test cases.
    let test_cases = [
        //                            //  Wrong password
        serde_json::json!({
            "email": random_email,
            "password": "wrong-password",
        }),
        //                            //  Wrong email
        serde_json::json!({
            "email": "wrong@email.com",
            "password": "password123",
        }),
        //                            //  Wrong email and password
        serde_json::json!({
            "email": "wrong@email.com",
            "password": "wrong-password",
        }),
    ];

    for test_case in test_cases {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
}