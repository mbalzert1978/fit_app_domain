use crate::prelude::*;
use crate::shared::ValueObject;
use crate::user_management::entities::user::abstractions::DateTimeProvider;

#[derive(Clone)]
pub(crate) struct JoinDate(String);

impl ValueObject for JoinDate {
    type Value = String;

    fn get_value(&self) -> &Self::Value {
        &self.0
    }
}

pub(crate) struct JoinDateCreator<Tdt> {
    datetime_provider: DateTimeProvider<Tdt>,
}

impl<Tdt> JoinDateCreator<Tdt>
where
    Tdt: 'static,
{
    pub(crate) fn new(datetime_provider: DateTimeProvider<Tdt>) -> Self {
        Self { datetime_provider }
    }
    pub(crate) fn parse(&self, join_date: &str) -> Result<JoinDate> {
        Ok(JoinDate(
            self.datetime_provider.as_iso8601(
                &self
                    .datetime_provider
                    .parse(join_date)
                    .map_err(|e| DomainError::IsoDateParsing(e.to_string()))?,
            ),
        ))
    }
    pub(crate) fn create(&self) -> JoinDate {
        let now = self.datetime_provider.utc_now();
        JoinDate(self.datetime_provider.as_iso8601(&now))
    }
}
