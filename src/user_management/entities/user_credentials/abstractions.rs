use std::sync::Arc;

pub type EmailValidator = fn(&str) -> bool;

pub type Hasher = Arc<dyn _Hasher>;
pub trait _Hasher {
    fn hash_password(&self, password: &str) -> String;
    fn verify_password(&self, hash: &str, password: &str) -> bool;
}
