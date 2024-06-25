use super::value_objects::account::AccountType;
use super::value_objects::email::Email;
use super::value_objects::id::DomainId;
use super::value_objects::joined_at::JoinDate;
use super::value_objects::password::Password;

#[derive(Clone)]
pub(crate) struct User<Tid> {
    pub(crate) user_id: DomainId<Tid>,
    pub(crate) email: Email,
    pub(crate) password: Password,
    pub(crate) user_name: Option<String>,
    pub(crate) account_type: AccountType,
    pub(crate) join_date: JoinDate,

    // Relationships
    pub(crate) following: Vec<DomainId<Tid>>,
    pub(crate) followers: Vec<DomainId<Tid>>,
    // possible relationships
    // pub(crate) posts: Vec<Id>,
    // pub(crate) comments: Vec<Id>,
    // pub(crate) likes: Vec<Id>,
    // pub(crate) liked_posts: Vec<Id>,
}
