use crate::data::{face::Face, identity::Identity, user::User, GLOBAL_DATA};
use railwork::action::Action;

#[derive(Default)]
pub(crate) struct FaceAdd {}

unsafe impl Send for FaceAdd {}

impl Action for FaceAdd {
    type Input = Vec<dlib_support::face::Face>;

    fn run(&mut self, input: Self::Input) {
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
