use super::identity::Identity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) name: String,
    pub(crate) uid: u32,
    pub identities: Vec<Identity>,
}

impl From<nix::unistd::User> for User {
    fn from(value: nix::unistd::User) -> Self {
        Self {
            name: value.name,
            uid: value.uid.into(),
            identities: vec![],
        }
    }
}

impl User {
    pub(crate) fn current() -> Self {
        let uid = nix::unistd::Uid::current();
        Self::from(nix::unistd::User::from_uid(uid).unwrap().unwrap())
    }
}
