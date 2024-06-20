#[derive(Debug)]
pub enum DomainError {
    Generic(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::Generic(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for DomainError {}
