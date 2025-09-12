use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
//                                    //  Encapsulate the actual value in a tuple struct.
//                                    //  To create an email use:
//                                    //    let email = Email.parse("mymail@domain.com");
pub struct Email(String);

impl Email {
    //                                //  This forces to use this parse method to create an email.
    pub fn parse(s: String) -> Result<Email, String> {
        //                            //  validate_email belongs to validator crate.
        //                            //  This function appears up to validator version 0.16.1
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email.", s))
        }
    }
}

//                                    //  This AsRef implementation is used to extract the value from Email
//                                    //  struct, with:
//                                    //    email.as_ref()
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    //                                //  This crate generate fake data.  It's used as dev dependency for
    //                                //  tests only.
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
       fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }        
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(valid_email.0).is_ok()
    }    

}