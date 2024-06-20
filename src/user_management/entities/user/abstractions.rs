use crate::prelude::*;
use std::sync::Arc;

pub type EmailValidator = fn(&str) -> bool;

pub type Hasher = Arc<dyn _Hasher>;
pub trait _Hasher {
    fn hash_password(&self, password: &str) -> String;
    fn verify_password(&self, hash: &str, password: &str) -> bool;
}

pub type DateTimeProvider<Tdt> = Arc<dyn _DateTimeProvider<DateTime = Tdt>>;

pub trait _DateTimeProvider: 'static + Send + Sync {
    type DateTime;
    fn utc_now(&self) -> Self::DateTime;
    fn add_minutes(&self, duration: i64) -> Self::DateTime;
    fn timestamp(&self, date_time: &Self::DateTime) -> i64;
    fn as_iso8601(&self, date_time: &Self::DateTime) -> String;
    fn parse(&self, join_date: &str) -> Result<Self::DateTime>;
}
