use crate::prelude::*;

/// # Actor core
/// A core struct containing some data common to all actors of all kinds, be them environment actors
/// or characters.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActorCore {
    pub identity: Identity,
    pub placement: Placement,
}

impl HasPlacement for ActorCore {
    fn placement(&self) -> &Placement {
        &self.placement
    }

    fn placement_mut(&mut self) -> &mut Placement {
        &mut self.placement
    }
}

impl Identifiable for ActorCore {
    fn identity(&self) -> Identity {
        self.identity
    }
}
