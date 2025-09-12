use crate::domain::{Email, Password, User};

#[async_trait::async_trait]
pub trait UserStore {
    //                                //  DONE-TODO:
    //                                //  Add the `add_user`, `get_user`, and `validate_user` methods.
    //                                //  Make sure all methods are async so we can use async user stores in
    //                                //  the future
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;

    async fn validate_user(&self, email: &Email, password: &Password) 
        -> Result<(), UserStoreError>;

}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug)]
pub enum BannedTokenStoreError {
    UnexpectedError,
}
