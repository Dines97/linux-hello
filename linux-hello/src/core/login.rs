use crate::data::user::User;

pub(crate) struct Login<'a> {
    data: std::sync::RwLockReadGuard<'a, crate::data::Data>,
}

impl<'a> Default for Login<'a> {
    fn default() -> Self {
        Self {
            data: crate::data::read(),
        }
    }
}

impl<'a> Login<'a> {
    pub(crate) fn run(&self, recorded_faces: Vec<dlib_support::face::Face>) -> Option<u32> {
        recorded_faces.iter().find_map(|face| {
            self.data.users.iter().find_map(|user| {
                if self.compare_face_and_user(face, user) {
                    Some(user.uid)
                } else {
                    None
                }
            })
        })
    }

    fn compare_face_and_user(&self, recorded_face: &dlib_support::face::Face, user: &User) -> bool {
        user.identities.iter().any(|identity| {
            identity
                .faces
                .iter()
                .any(|face| euclidean_distance(&recorded_face.face_descriptor.to_vec(), &face.vec) < 0.5)
        })
    }
}

fn euclidean_distance(left: &[f32], right: &[f32]) -> f32 {
    assert!(left.len() == right.len(), "Vectors must have the same length");

    let sum_of_squares: f32 = std::iter::zip(left, right).map(|(&x, &y)| (x - y).powi(2)).sum();

    sum_of_squares.sqrt()
}
