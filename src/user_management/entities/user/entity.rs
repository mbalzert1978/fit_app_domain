use crate::shared::domain_id::DomainId;

use super::value_objects::account::AccountType;
use super::value_objects::joined_at::JoinDate;

#[derive(Clone)]
pub(crate) struct User<Tid> {
    user_id: DomainId<Tid>,
    user_name: Option<String>,
    account_type: AccountType,
    join_date: JoinDate,

    // Relationships
    following: Vec<DomainId<Tid>>,
    followers: Vec<DomainId<Tid>>,
    // possible relationships
    // pub(crate) posts: Vec<Id>,
    // pub(crate) comments: Vec<Id>,
    // pub(crate) likes: Vec<Id>,
    // pub(crate) liked_posts: Vec<Id>,
}

impl<Tid> User<Tid> {
    pub(crate) fn new(
        user_id: DomainId<Tid>,
        user_name: Option<String>,
        account_type: AccountType,
        join_date: JoinDate,
        following: Vec<DomainId<Tid>>,
        followers: Vec<DomainId<Tid>>,
    ) -> Self {
        Self {
            user_id,
            user_name,
            account_type,
            join_date,
            following,
            followers,
        }
    }
}
