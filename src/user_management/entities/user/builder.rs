use std::fmt::Display;
use std::str::FromStr;

use crate::prelude::*;
use crate::shared::domain_id::DomainId;

use super::abstractions::DateTimeProvider;
use super::abstractions::IdProvider;
use super::entity::User;
use super::value_objects::account::AccountType;
use super::value_objects::joined_at::JoinDate;
use super::value_objects::joined_at::JoinDateCreator;

pub(crate) struct UserBuilder<Tdt, Tid> {
    datetime_provider: DateTimeProvider<Tdt>,
    user_id: DomainId<Tid>,
    user_name: Option<String>,
    account_type: AccountType,
    join_date: JoinDate,
    // Relationships
    following: Vec<DomainId<Tid>>,
    followers: Vec<DomainId<Tid>>,
}

impl<Tdt, Tid> UserBuilder<Tdt, Tid>
where
    Tdt: 'static,
    Tid: FromStr + PartialEq,
    <Tid as FromStr>::Err: Display,
{
    pub(crate) fn new(
        datetime_provider: DateTimeProvider<Tdt>,
        id_provider: IdProvider<Tid>,
    ) -> Self {
        Self {
            datetime_provider: datetime_provider.clone(),
            user_id: DomainId::new(id_provider),
            user_name: None,
            account_type: AccountType::Free,
            join_date: JoinDateCreator::new(datetime_provider).create(),
            following: Vec::new(),
            followers: Vec::new(),
        }
    }
    pub(crate) fn add_user_id(mut self, user_id: &str) -> Result<Self> {
        self.user_id = DomainId::from_str(user_id)?;
        Ok(self)
    }
    pub(crate) fn add_user_name(mut self, user_name: &str) -> Result<Self> {
        self.user_name = Some(user_name.to_string());
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
    pub(crate) fn add_following(mut self, following: Vec<DomainId<Tid>>) -> Result<Self> {
        self.following = following;
        Ok(self)
    }
    pub(crate) fn add_followers(mut self, followers: Vec<DomainId<Tid>>) -> Result<Self> {
        self.followers = followers;
        Ok(self)
    }
    pub(crate) fn build(self) -> Result<User<Tid>> {
        Ok(User::new(
            self.user_id,
            self.user_name,
            self.account_type,
            self.join_date,
            self.following,
            self.followers,
        ))
    }
}
