use super::value_objects::account::AccountType;
use super::value_objects::email::Email;
use super::value_objects::joined_at::JoinDate;
use super::value_objects::password::Password;

use crate::shared::id::Id;

#[derive(Clone)]
pub(crate) struct User {
    pub(crate) user_id: Id,
    pub(crate) user_name: String,
    pub(crate) email: Email,
    pub(crate) password: Password,
    pub(crate) account_type: AccountType,
    pub(crate) join_date: JoinDate,
}
