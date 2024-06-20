use abstractions::DateTimeProvider;
use abstractions::EmailValidator;
use value_objects::account::AccountType;
use value_objects::email::Email;
use value_objects::email::EmailParser;
use value_objects::joined_at::JoinDate;
use value_objects::joined_at::JoinDateCreator;
use value_objects::password::Password;

use crate::prelude::*;
use crate::shared::id::Id;

pub(crate) mod abstractions;
pub(crate) mod value_objects;

#[derive(Clone)]
pub(crate) struct User {
    user_id: Id,
    user_name: String,
    email: Email,
    password: Password,
    account_type: AccountType,
    join_date: JoinDate,
}

pub(crate) struct UserBuilder<Tdt> {
    validator: EmailValidator,
    datetime_provider: DateTimeProvider<Tdt>,
    user_id: Option<Id>,
    user_name: Option<String>,
    email: Option<Email>,
    password: Option<Password>,
    account_type: AccountType,
    join_date: Option<JoinDate>,
}

impl<Tdt> UserBuilder<Tdt>
where
    Tdt: 'static,
{
    pub(crate) fn new(validator: EmailValidator, datetime_provider: DateTimeProvider<Tdt>) -> Self {
        Self {
            validator,
            datetime_provider,
            user_id: None,
            user_name: None,
            email: None,
            password: None,
            account_type: AccountType::Free,
            join_date: None,
        }
    }
    pub(crate) fn update(
        validator: EmailValidator,
        datetime_provider: DateTimeProvider<Tdt>,
        user: &User,
    ) -> Self {
        Self {
            validator,
            datetime_provider,
            user_id: Some(user.user_id.clone()),
            user_name: Some(user.user_name.to_string()),
            email: Some(user.email.clone()),
            password: Some(user.password.clone()),
            account_type: user.account_type.clone(),
            join_date: Some(user.join_date.clone()),
        }
    }
    pub(crate) fn with_user_id(mut self, user_id: &str) -> Result<Self> {
        self.user_id = Some(user_id.parse::<Id>()?);
        Ok(self)
    }
    pub(crate) fn create_user_id(mut self) -> Result<Self> {
        self.user_id = Some(Id::new());
        Ok(self)
    }
    pub(crate) fn with_user_name(mut self, user_name: &str) -> Result<Self> {
        self.user_name = Some(user_name.to_string());
        Ok(self)
    }
    pub(crate) fn with_email(mut self, email: &str) -> Result<Self> {
        self.email = Some(EmailParser::new(self.validator).parse(email)?);
        Ok(self)
    }
    pub(crate) fn with_password(mut self, password: &str) -> Result<Self> {
        self.password = Some(password.parse::<Password>()?);
        Ok(self)
    }
    pub(crate) fn with_account_type(mut self, account_type: AccountType) -> Result<Self> {
        self.account_type = account_type;
        Ok(self)
    }
    pub(crate) fn with_join_date(mut self, join_date: &str) -> Result<Self> {
        self.join_date =
            Some(JoinDateCreator::new(self.datetime_provider.clone()).parse(join_date)?);
        Ok(self)
    }
    pub(crate) fn create_join_date(mut self) -> Result<Self> {
        self.join_date = Some(JoinDateCreator::new(self.datetime_provider.clone()).create());
        Ok(self)
    }
}
