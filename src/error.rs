#[derive(Debug)]
pub enum DomainError {
    Generic(String),
    InvalidEmail(String),

    Uuid(String),

    PasswordTooShort,
    PasswordMissingUppercase,
    PasswordMissingDigit,
    PasswordMissingSymbol,
    IsoDateParsing(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::Generic(s) => write!(f, "{}", s),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl std::error::Error for DomainError {}

impl From<uuid::Error> for DomainError {
    fn from(value: uuid::Error) -> Self {
        DomainError::Uuid(value.to_string())
    }
}
