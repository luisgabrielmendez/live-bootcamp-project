use std::collections::HashMap;

use crate::domain::{
    Email,
    Password,
    User,
    UserStore,
    UserStoreError
};


//                                    //  DONE-TODO:
//                                    //  Create a new struct called `HashmapUserStore` containing a users`
//                                    //  field which stores a `HashMap` of email `String`s mapped to
//                                    //  `User` objects.
//                                    //  Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        //                            //  DONE->TODO:
        //                            //  Purpose: Insert new user.
        //                            //  Return: Result
        //                            //    Ok(()), if the user was inserted correctly into the hashmap.
        //                            //    Err(UserStoreError::UserAlreadyExists), if the user already exists.
        //                            //  
        if self.users.contains_key(&user.email) {
            //                        //  Abort switch. Exist, return error.
            return Err(UserStoreError::UserAlreadyExists)
        }
        //                            //  Not exist. Insert, return Ok.
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    //                                //  DONE->TODO:
    //                                //  Implement a public method called `get_user`, which takes an inmutable
    //                                //  reference to self and an email string slice as arguments.
    //                                //  This function should return a `Result` type containing either a
    //                                //  `User` object or a `UserStoreError`.
    //                                //  Return `UserStoreError::UserNotFound` if the user can not be found.
    //                                //  
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        //                            //  Purpose: Get existing user. 
        //                            //  Return: Result
        //                            //    Ok(User), if the user was found.
        //                            //    Err(UserStoreError::UserNotFound), if the user can not be found.
        //                            //  
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    //                                //  DONE->TODO:
    //                                //  Implement a public method called `validate_user`, which takes an 
    //                                //  immutable reference to self, an email string slice, and a password
    //                                //  string slice as arguments.
    //                                //  `validate_user` should return a `Result` type containing either a
    //                                //  unit type `()` if the email/password passed in match an existing user,
    //                                //  or a `UserStoreError`.
    //                                //  Return `UserStoreError::UserNotFound` if the user can not be found.
    //                                //  Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    //                                //  
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password
    ) -> Result<(), UserStoreError>
    {
        //                            //  Purpose: Validate user credentials.
        //                            //  Return: Result
        //                            //  Ok(())
        //                            //  Err(UserStoreError::UserNotFound) if the user can not be found.
        //                            //  Err(UserStoreError::InvalidCredentials) if the password is incorrect.
        //                            //  
        //                            //  This function can be refactored to reuse get function:
        //                            //    let user = get_user(email)?;
        //                            //    if user.password.eq(password) {
        //                            //        Ok(())
        //                            //    } else {
        //                            //        Err(UserStoreError::InvalidCreadentials)
        //                            //    }
        //                            //  
        //                            //  The following implementation is the one that Bogdan did.
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            },
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

//                                    //  DONE->TODO:
//                                    //  Add unit tests for your `HashmapUserStore` implementation.
#[cfg(test)]
mod tests {
    use super::*;

    fn create_vars() -> (HashmapUserStore, Email, Password, User) {
        let user = User {
            email: Email::parse("test@example.com".to_owned()).unwrap(),
            password: Password::parse("password".to_owned()).unwrap(),
            requires_2fa: false,
        };
        (HashmapUserStore::default(),
         user.email.clone(), 
         user.password.clone(),
         user
        )
    }

    //                                //  The basic structure in all tests is:
    //                                //  1. Initialize variables to execute test.
    //                                //  2. Perform tests using those variables for each possible result.

    #[tokio::test]
    async fn test_add_user() {
        //                            //  DONE-TODO:
        //                            //  Possible results:
        //                            //  - Adding an inexisting user -> Ok(())
        //                            //  - Adding an existing user -> Err(())

        //                            //  Initialize variables to execute test
        let (mut user_store, _, _, user) = create_vars();

        //                            //  Test adding a new user
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        //                            //  Test adding the existing user
        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        //                            //  DONE-TODO:
        //                            //  Possible results:
        //                            //  - Getting an existing user -> Ok(())
        //                            //  - Getting an inexisting user -> Err(())

        //                            //  Initialize environment for test
        let (mut user_store, email, _, user) = create_vars();
        user_store.users.insert(email.clone(), user.clone());

        //                            //  Test getting a user that exists
        let result = user_store.get_user(&email).await;
        assert_eq!(result, Ok(user));

        //                            //  Test getting a user that doesn't exists.
        let result = user_store
            .get_user(&Email::parse("nonexistent@example.com".to_owned()).unwrap())
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
        
    }

    #[tokio::test]
    async fn test_validate_user() {
        //                            //  DONE-TODO:
        //                            //  Possible results:
        //                            //  - Validate an existing user with correct password -> Ok(())
        //                            //  - Validate an existing user with incorrect password -> Err(())
        //                            //  - Validate an inexisting user -> Err(())

        //                            //  Initialize environment for test
        let (mut user_store, email, password, user) = create_vars();
        user_store.users.insert(email.clone(), user.clone());

        //                            //  Test validating a user that exist with correct password
        let result = user_store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        //                            //  Test validating a user that exist with incorrect password
        let wrong_password = Password::parse("wrongpassword".to_owned()).unwrap();
        let result = user_store
            .validate_user(&email, &wrong_password)
            .await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        //                            //  Test validating a user that doesn't exist
        let result = user_store
            .validate_user(&Email::parse("nonexistent@example.com".to_owned()).unwrap(), &password).await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}