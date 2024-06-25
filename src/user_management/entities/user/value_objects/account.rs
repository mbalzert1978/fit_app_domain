use crate::shared::ValueObject;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub(crate) enum AccountType {
    Free,
    Monthly,
    Lifetime,
}

impl ValueObject for AccountType {
    type Value = str;

    fn get_value(&self) -> &Self::Value {
        match self {
            AccountType::Free => "Free",
            AccountType::Monthly => "Monthly",
            AccountType::Lifetime => "Lifetime",
        }
    }
}
