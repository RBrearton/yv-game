use crate::prelude::*;

/// # Action core
/// A core struct containing some data common to all actions.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionCore {
    pub action_id: Identity,
    pub performer_id: Identity,
    pub target_id: Option<Identity>,
}

impl Identifiable for ActionCore {
    fn identity(&self) -> Identity {
        self.action_id
    }
}

impl HasPerformer for ActionCore {
    fn performer(&self) -> &Identity {
        &self.performer_id
    }
}

impl HasTarget for ActionCore {
    fn target(&self) -> Option<&Identity> {
        self.target_id.as_ref()
    }
}
