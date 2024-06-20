use std::str::FromStr;

use crate::prelude::*;

use super::abstractions::ValueObject;
#[derive(Clone)]
pub(crate) struct Id(uuid::Uuid);

impl Id {
    pub(crate) fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }
}

impl ValueObject for Id {
    type Value = uuid::Uuid;
    fn get_value(&self) -> &Self::Value {
        &self.0
    }
}

impl FromStr for Id {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(uuid::Uuid::parse_str(s)?))
    }
}
