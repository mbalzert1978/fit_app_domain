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

    // Relationships
    pub(crate) following: Vec<Id>,
    pub(crate) followers: Vec<Id>,
    // possible relationships
    // pub(crate) posts: Vec<Id>,
    // pub(crate) comments: Vec<Id>,
    // pub(crate) likes: Vec<Id>,
    // pub(crate) liked_posts: Vec<Id>,
}
