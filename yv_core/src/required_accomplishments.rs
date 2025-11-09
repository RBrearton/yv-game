use crate::prelude::*;

/// # Required accomplishments
/// This is a simple struct that stores a small, fixed-size number of accomplishments that can
/// be part of a Requirements struct.
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequiredAccomplishments {
    accomplishment_0: Option<Accomplishment>,
    accomplishment_1: Option<Accomplishment>,
}

impl IntoIterator for RequiredAccomplishments {
    type Item = Accomplishment;
    type IntoIter = std::iter::Chain<
        std::option::IntoIter<Accomplishment>,
        std::option::IntoIter<Accomplishment>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.accomplishment_0
            .into_iter()
            .chain(self.accomplishment_1)
    }
}

impl From<Accomplishment> for RequiredAccomplishments {
    fn from(accomplishment: Accomplishment) -> Self {
        Self {
            accomplishment_0: Some(accomplishment),
            accomplishment_1: None,
        }
    }
}

impl From<(Accomplishment, Accomplishment)> for RequiredAccomplishments {
    fn from((accomplishment_0, accomplishment_1): (Accomplishment, Accomplishment)) -> Self {
        Self {
            accomplishment_0: Some(accomplishment_0),
            accomplishment_1: Some(accomplishment_1),
        }
    }
}
