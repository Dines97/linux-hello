use crate::data::{face::Face, identity::Identity, user::User};

pub(crate) struct FaceManager<'lock> {
    data: std::sync::RwLockWriteGuard<'lock, crate::data::Data>,
}

impl<'lock> Default for FaceManager<'lock> {
    fn default() -> Self {
        Self {
            data: crate::data::GLOBAL_DATA.write().unwrap(),
        }
    }
}

impl<'lock> FaceManager<'lock> {
    pub(crate) fn add_identity(&mut self, input: Vec<dlib_support::face::Face>) {
        let mut new_user = User::current();
        let target_user = self.data.users.iter_mut().find(|x| x.name == new_user.name);

        let identity: Identity = Identity {
            name: String::from("placeholder"),
            faces: input
                .iter()
                .map(|x| Face {
                    vec: x.face_descriptor.to_vec(),
                })
                .collect(),
        };

        match target_user {
            Some(user) => user.identities.push(identity),
            None => {
                new_user.identities.push(identity);
                self.data.users.push(new_user);
            }
        }

        self.data.serialize();
    }
}
