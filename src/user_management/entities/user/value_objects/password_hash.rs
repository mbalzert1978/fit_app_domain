use super::password::Password;
#[derive(Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(password: &Password) -> Self {
        //salting here
        todo!()
    }

    pub fn as_str(&self) -> &str {
        AsRef::as_ref(self)
    }

    pub fn into_string(self) -> String {
        Into::into(self)
    }

    pub fn verify(&self, password: &str) -> bool {
        // compare here
        todo!()
    }
}

impl AsRef<str> for PasswordHash {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<PasswordHash> for String {
    fn from(value: PasswordHash) -> Self {
        value.0
    }
}
