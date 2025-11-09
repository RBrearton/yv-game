use crate::prelude::*;

/// # Action core
/// A core struct containing some data common to all actions.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionCore {
    pub action_id: Identity,
    pub performer: Identity,
}

impl Identifiable for ActionCore {
    fn identity(&self) -> Identity {
        self.action_id
    }
}
