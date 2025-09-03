#[derive(Default, Clone, PartialEq, Debug)]
pub struct User {
    //                                //  DONE-TODO:
    //                                //  The User struct should contain 3 fields:
    //                                //    - email, which is a String.
    //                                //    - password, which is also a String.
    //                                //    - requires_2fa, which is a boolean. 
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    //                                //  DONE-TODO:
    //                                //  Add a constructor function called `new`
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa
        }
    }
}