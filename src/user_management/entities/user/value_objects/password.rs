use std::str::FromStr;

use super::{password_hash::PasswordHash, ValueObject};

use crate::prelude::*;

const MIN_LENGTH: usize = 12;
const DIGIT_BASE: u32 = 10;

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Password(String);

impl Password {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_string(self) -> String {
        Into::into(self)
    }

    pub fn to_hash(&self) -> PasswordHash {
        PasswordHash::new(self)
    }
}

impl FromStr for Password {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() < MIN_LENGTH {
            return Err(DomainError::PasswordTooShort);
        }
        if !s.chars().any(|c| c.is_uppercase()) {
            return Err(DomainError::PasswordMissingUppercase);
        }
        if !s.chars().any(|c| c.is_digit(DIGIT_BASE)) {
            return Err(DomainError::PasswordMissingDigit);
        }
        if !s.chars().any(|c| !c.is_alphanumeric()) {
            return Err(DomainError::PasswordMissingSymbol);
        }

        Ok(Self(s.into()))
    }
}

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

impl ValueObject for Password {
    type Value = String;

    fn get_value(&self) -> &Self::Value {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn password_validation_when_empty_then_error_password_too_short() {
        let input = "";
        assert!(matches!(
            input.parse::<Password>(),
            Err(DomainError::PasswordTooShort)
        ));
    }

    #[test]
    fn password_validation_when_no_uppercase_then_error_password_missing_uppercase() {
        assert!(matches!(
            "thispasswordhasnoupper".parse::<Password>(),
            Err(DomainError::PasswordMissingUppercase)
        ));
    }

    #[test]
    fn password_validation_when_no_digit_then_error_password_missing_digit() {
        assert!(matches!(
            "Abcdefghijklmnopqrst".parse::<Password>(),
            Err(DomainError::PasswordMissingDigit)
        ));
    }

    #[test]
    fn password_validation_when_no_symbol_then_error_password_missing_symbol() {
        assert!(matches!(
            "Abcdefghijklmnopqrst123".parse::<Password>(),
            Err(DomainError::PasswordMissingSymbol)
        ));
    }

    #[test]
    fn password_validation_when_valid_password_should_return_password_instance() {
        assert!(matches!(
            "01234-5678-Abcd-valid".parse::<Password>(),
            Ok(Password(pw)) if pw == "01234-5678-Abcd-valid"
        ));
    }
}
