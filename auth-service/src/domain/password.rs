#[derive(Debug, Clone, PartialEq)]
//                                    //  Encapsulate the actual value in a tuple struct.
//                                    //  To create a password use:
//                                    //    let password = Password.parse("password123");

pub struct Password(String);

impl Password {
    //                                //  This forces to use this parse method to create a password.
    pub fn parse(s: String) -> Result<Password, String> {
        if validate_password(&s) {
            Ok(Self(s))
        } else {
            Err("Failed to parse string to a Password type".to_owned())
        }
    }
}

fn validate_password(s: &str) -> bool {
    s.len() >= 8
}

//                                    //  This AsRef implementation is used to extract the value from Password
//                                    //  struct, with:
//                                    //    password.as_ref()
impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    //                                //  This crate generate fake data.  It's used as dev dependency for
    //                                //  tests only.
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let password = "".to_owned();
        assert!(Password::parse(password).is_err());
    }
    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = "1234567".to_owned();
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..30).fake_with_rng(g);
            Self(password)
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(valid_password.0).is_ok()
    }
}