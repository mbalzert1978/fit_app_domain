use std::fmt::Display;
use std::str::FromStr;

use crate::prelude::*;
use crate::user_management::entities::user::abstractions::IdProvider;

use super::ValueObject;

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct Id<Tid>(Tid);

impl<Tid> Id<Tid> {
    pub(crate) fn new(id_provider: IdProvider<Tid>) -> Self {
        Self(id_provider.now_v7())
    }
}

impl<Tid> ValueObject for Id<Tid> {
    type Value = Tid;
    fn get_value(&self) -> &Self::Value {
        &self.0
    }
}

impl<Tid> FromStr for Id<Tid>
where
    Tid: FromStr,
    <Tid as FromStr>::Err: Display,
{
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.parse::<Tid>()
                .map_err(|e| DomainError::InvalidId(e.to_string()))?,
        ))
    }
}
