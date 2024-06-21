use crate::prelude::*;
use crate::user_management::entities::user::abstractions::EmailValidator;

use super::ValueObject;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Email(String);

impl ValueObject for Email {
    type Value = String;
    fn get_value(&self) -> &Self::Value {
        &self.0
    }
}

pub(crate) struct EmailParser {
    validator: EmailValidator,
}

impl EmailParser {
    pub(crate) fn new(validator: EmailValidator) -> Self {
        Self { validator }
    }
    pub(crate) fn parse(&self, email: &str) -> Result<Email> {
        if !(self.validator)(email) {
            return Err(DomainError::InvalidEmail(email.to_string()));
        }
        Ok(Email(email.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_parser_when_email_is_invalid_should_return_error() {
        let result = EmailParser::new(|_| false).parse("invalid_email");
        assert!(matches!(result, Err(DomainError::InvalidEmail(_))));
    }

    #[test]
    fn test_email_parser_when_email_is_valid_should_return_email_instance() {
        let result = EmailParser::new(|_| true).parse("valid_email");
        assert!(matches!(result, Ok(Email(_))));
    }
}
