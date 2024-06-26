use crate::prelude::*;
use std::sync::Arc;

pub type DateTimeProvider<Tdt> = Arc<dyn _DateTimeProvider<DateTime = Tdt>>;

pub trait _DateTimeProvider: 'static + Send + Sync {
    type DateTime;
    fn utc_now(&self) -> Self::DateTime;
    fn add_minutes(&self, duration: i64) -> Self::DateTime;
    fn timestamp(&self, date_time: &Self::DateTime) -> i64;
    fn as_iso8601(&self, date_time: &Self::DateTime) -> String;
    fn parse(&self, join_date: &str) -> Result<Self::DateTime>;
}

pub type IdProvider<Tid> = Arc<dyn _IdProvider<Id = Tid>>;

pub trait _IdProvider: Send + Sync {
    type Id;
    fn now_v7(&self) -> Self::Id;
}
