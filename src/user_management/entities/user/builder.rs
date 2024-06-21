use crate::prelude::*;
use crate::shared::id::Id;

use super::abstractions::DateTimeProvider;
use super::abstractions::EmailValidator;
use super::value_objects::account::AccountType;
use super::value_objects::email::Email;
use super::value_objects::email::EmailParser;
use super::value_objects::joined_at::JoinDate;
use super::value_objects::joined_at::JoinDateCreator;
use super::value_objects::password::Password;

pub(crate) struct UserBuilder<Tdt> {
    validator: EmailValidator,
    datetime_provider: DateTimeProvider<Tdt>,
    user_id: Option<Id>,
    user_name: Option<String>,
    email: Option<Email>,
    password: Option<Password>,
    account_type: AccountType,
    join_date: JoinDate,
    // Relationships
    following: Vec<Id>,
    followers: Vec<Id>,
}

impl<Tdt> UserBuilder<Tdt>
where
    Tdt: 'static,
{
    pub(crate) fn new(validator: EmailValidator, datetime_provider: DateTimeProvider<Tdt>) -> Self {
        Self {
            validator,
            datetime_provider: datetime_provider.clone(),
            user_id: None,
            user_name: None,
            email: None,
            password: None,
            account_type: AccountType::Free,
            join_date: JoinDateCreator::new(datetime_provider).create(),
            following: Vec::new(),
            followers: Vec::new(),
        }
    }

    pub(crate) fn add_user_id(mut self, user_id: Option<&str>) -> Result<Self> {
        self.user_id = match user_id {
            Some(id) => Some(id.parse::<Id>()?),
            None => Some(Id::new()),
        };
        Ok(self)
    }
    pub(crate) fn add_user_name(mut self, user_name: &str) -> Result<Self> {
        self.user_name = Some(user_name.to_string());
        Ok(self)
    }
    pub(crate) fn add_email(mut self, email: &str) -> Result<Self> {
        self.email = Some(EmailParser::new(self.validator).parse(email)?);
        Ok(self)
    }
    pub(crate) fn add_password(mut self, password: &str) -> Result<Self> {
        self.password = Some(password.parse::<Password>()?);
        Ok(self)
    }
    pub(crate) fn add_account_type(mut self, account_type: AccountType) -> Result<Self> {
        self.account_type = account_type;
        Ok(self)
    }
    pub(crate) fn add_join_date(mut self, join_date: &str) -> Result<Self> {
        self.join_date = JoinDateCreator::new(self.datetime_provider.clone()).parse(join_date)?;
        Ok(self)
    }
    pub(crate) fn add_following(mut self, following: Vec<Id>) -> Result<Self> {
        self.following = following;
        Ok(self)
    }
    pub(crate) fn add_followers(mut self, followers: Vec<Id>) -> Result<Self> {
        self.followers = followers;
        Ok(self)
    }
}
