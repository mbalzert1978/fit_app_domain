use std::fmt::Display;
use std::str::FromStr;

use crate::prelude::*;

use super::abstractions::DateTimeProvider;
use super::abstractions::EmailValidator;
use super::abstractions::IdProvider;
use super::entity::User;
use super::value_objects::account::AccountType;
use super::value_objects::email::Email;
use super::value_objects::email::EmailParser;
use super::value_objects::id::Id;
use super::value_objects::joined_at::JoinDate;
use super::value_objects::joined_at::JoinDateCreator;
use super::value_objects::password::Password;

pub(crate) struct UserBuilder<Tdt, Tid> {
    validator: EmailValidator,
    datetime_provider: DateTimeProvider<Tdt>,
    user_id: Id<Tid>,
    user_name: Option<String>,
    email: Option<Email>,
    password: Option<Password>,
    account_type: AccountType,
    join_date: JoinDate,
    // Relationships
    following: Vec<Id<Tid>>,
    followers: Vec<Id<Tid>>,
}

impl<Tdt, Tid> UserBuilder<Tdt, Tid>
where
    Tdt: 'static,
    Tid: FromStr,
    <Tid as FromStr>::Err: Display,
{
    pub(crate) fn new(
        validator: EmailValidator,
        datetime_provider: DateTimeProvider<Tdt>,
        id_provider: IdProvider<Tid>,
    ) -> Self {
        Self {
            validator,
            datetime_provider: datetime_provider.clone(),
            user_id: Id::new(id_provider),
            user_name: None,
            email: None,
            password: None,
            account_type: AccountType::Free,
            join_date: JoinDateCreator::new(datetime_provider).create(),
            following: Vec::new(),
            followers: Vec::new(),
        }
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
    pub(crate) fn build(self) -> Result<User<Tid>> {
        let Some(email) = self.email else {
            return Err(DomainError::MissingRequiredField("email".to_string()));
        };
        let Some(password) = self.password else {
            return Err(DomainError::MissingRequiredField("password".to_string()));
        };
        Ok(User {
            user_id: self.user_id,
            email,
            password,
            user_name: self.user_name,
            account_type: self.account_type,
            join_date: self.join_date,
            following: self.following,
            followers: self.followers,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::user_management::entities::user::abstractions::{_DateTimeProvider, _IdProvider};
    use crate::user_management::entities::user::value_objects::ValueObject;

    use super::*;

    struct MockProvider;

    impl _DateTimeProvider for MockProvider {
        type DateTime = String;

        fn utc_now(&self) -> Self::DateTime {
            "2023-01-01T00:00:00Z".to_string()
        }

        fn add_minutes(&self, duration: i64) -> Self::DateTime {
            "2023-01-02T00:00:00Z".to_string()
        }

        fn timestamp(&self, date_time: &Self::DateTime) -> i64 {
            123456
        }

        fn as_iso8601(&self, date_time: &Self::DateTime) -> String {
            "2023-01-01T00:00:00Z".to_string()
        }

        fn parse(&self, join_date: &str) -> Result<Self::DateTime> {
            Ok("2023-01-01T00:00:00Z".to_string())
        }
    }

    impl _IdProvider for MockProvider {
        type Id = String;

        fn now_v7(&self) -> Self::Id {
            "123e4567-e89b-12d3-a456-426614174000".to_string()
        }
    }

    const MOCK_VALIDATOR: fn(&str) -> bool = |_| true;

    #[test]
    fn user_builder_when_new_called_should_return_builder_instance_with_acc_type_on_free_and_join_date_set_to_now_with_provider(
    ) {
        let builder = UserBuilder::new(
            MOCK_VALIDATOR,
            Arc::new(MockProvider),
            Arc::new(MockProvider),
        );
        assert_eq!(*builder.user_id.get_value(), MockProvider.now_v7());
        assert!(builder.user_name.is_none());
        assert!(builder.email.is_none());
        assert!(builder.password.is_none());
        assert_eq!(builder.account_type, AccountType::Free);
        assert_eq!(builder.join_date.get_value(), "2023-01-01T00:00:00Z");
        assert!(builder.following.is_empty());
        assert!(builder.followers.is_empty());
    }

    #[test]
    fn user_builder_when_no_id_is_given_should_create_id_with_provider_and_store_in_instance() {
        let builder = UserBuilder::new(
            MOCK_VALIDATOR,
            Arc::new(MockProvider),
            Arc::new(MockProvider),
        );

        assert_eq!(
            builder.user_id.get_value(),
            "123e4567-e89b-12d3-a456-426614174000"
        );
    }

    #[test]
    fn user_builder_when_user_name_is_called_should_store_name_in_instance() {
        let builder = UserBuilder::new(
            MOCK_VALIDATOR,
            Arc::new(MockProvider),
            Arc::new(MockProvider),
        )
        .add_user_name("test_user")
        .unwrap();

        assert_eq!(builder.user_name.unwrap(), "test_user");
    }

    #[test]
    fn user_builder_when_add_email_is_called_should_validate_email_with_provider_and_store_in_instance(
    ) {
        let builder = UserBuilder::new(
            MOCK_VALIDATOR,
            Arc::new(MockProvider),
            Arc::new(MockProvider),
        )
        .add_email("test@example.com")
        .unwrap();

        assert_eq!(builder.email.unwrap().get_value(), "test@example.com");
    }

    #[test]
    fn user_builder_when_add_password_is_called_should_validate_password_and_store_in_instance() {
        let builder = UserBuilder::new(
            MOCK_VALIDATOR,
            Arc::new(MockProvider),
            Arc::new(MockProvider),
        )
        .add_password("Valid_password_is_123!")
        .unwrap();

        assert_eq!(
            builder.password.unwrap().get_value(),
            "Valid_password_is_123!"
        );
    }

    #[test]
    fn user_builder_when_add_account_type_is_called_should_store_in_instance() {
        let builder = UserBuilder::new(
            MOCK_VALIDATOR,
            Arc::new(MockProvider),
            Arc::new(MockProvider),
        )
        .add_account_type(AccountType::Monthly)
        .unwrap();

        assert_eq!(builder.account_type, AccountType::Monthly);
    }
}
