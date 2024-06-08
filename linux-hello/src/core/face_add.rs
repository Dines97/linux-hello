use crate::data::{face::Face, identity::Identity, user::User, GLOBAL_DATA};

#[derive(Default)]
pub(crate) struct FaceAdd {}

unsafe impl Send for FaceAdd {}

impl FaceAdd {
    pub(crate) fn run(&mut self, input: Vec<dlib_support::face::Face>) {
        let mut state = GLOBAL_DATA.write().unwrap();

        let mut new_user = User::current();
        let target_user = state.users.iter_mut().find(|x| x.name == new_user.name);

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
                state.users.push(new_user);
            }
        }

        state.serialize();
    }
}
