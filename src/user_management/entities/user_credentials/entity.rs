use crate::shared::domain_id::DomainId;

use super::value_objects::{email::Email, password::Password};
pub(crate) struct UserCredentials<Tid> {
    user_id: DomainId<Tid>,
    email: Email,
    password: Password,
}

impl<Tid> UserCredentials<Tid> {
    pub(crate) fn new(user_id: DomainId<Tid>, email: Email, password: Password) -> Self {
        Self {
            user_id,
            email,
            password,
        }
    }
}
