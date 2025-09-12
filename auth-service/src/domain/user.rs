use super::{Email, Password};

#[derive(Clone, PartialEq, Debug)]
pub struct User {
    //                                //  DONE-TODO:
    //                                //  The User struct should contain 3 fields:
    //                                //    - email, which is a String. -> Email
    //                                //    - password, which is also a String. -> Password
    //                                //    - requires_2fa, which is a boolean. 
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    //                                //  DONE-TODO:
    //                                //  Add a constructor function called `new`
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa
        }
    }
}