use crate::{helpers::{get_random_email, TestApp}};
use auth_service::{domain::ErrorResponse, routes::SignupResponse};

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new("signup - should_return_201_if_valid_input").await;

    let random_email = get_random_email();

        let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    //                                //  The signup route should return a 400 HTTP status code if an invalid
    //                                //  input is sent.
    //                                //  The input is considered invalid if:
    //                                //  - The email is empty or does not contain '@'
    //                                //  - The password is less than 8 characters
    
    //                                //  Create an array of invalid inputs. Then, iterate through the array
    //                                //  and make HTTP calls to the signup route. Assert a 400 HTTP status
    //                                //  code is returned.

    //                                //  PREPARE TEST ENVIRONMENT
    let app = TestApp::new("signup - should_return_400_if_invalid_input").await;

    let random_email = get_random_email();

    //                                //  Array of invalid inputs to test.
    let signup_bodies = vec![
        //                            //  No email
        serde_json::json!({
            "email": "",
            "password": "password123",
            "requires2FA": true
        }),
        //                            //  No password
        serde_json::json!({
            "email": random_email,
            "password": "",
            "requires2FA": true
        }),
        //                            //  No email and password
        serde_json::json!({
            "email": "",
            "password": "",
            "requires2FA": true
        }),
        //                            //  Invalid email
        serde_json::json!({
            "email": "invalid_email",
            "password": "password123",
            "requires2FA": true
        }),
        //                            //  Invalid password: Less that 8 characters
        serde_json::json!({
            "email": random_email,
            "password": "invalid",
            "requires2FA": true
        }),
        //                            //  The case left out:requires2FA is not boolean
    ];

    //                                //  Execute all tests
    for signup_body in signup_bodies {
        let response = app.post_signup(&signup_body).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", signup_body);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    //                                //  Call the signup route twice. The second request should fail with a
    //                                //  409 HTTP status code

    //                                //  PREPARE TEST ENVIRONMENT
    let app = TestApp::new("signup - should_return_409_if_email_already_exists").await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    //                                //  CALL SIGNUP TWICE
    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 409);

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to UserBody")
            .error,
        "User already exists".to_owned()
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new("signup - should_return_422_if_malformed_input").await;
    let random_email = get_random_email();
    let test_cases = vec![
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
            "requires2FA": "true"
        }),
        serde_json::json!({}),
    ];
    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}