use crate::prelude::*;

/// # Oak
/// An oak tree is a type of tree that can be chopped down.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
pub struct Oak {
    pub core: ActorCore,
}

impl Describable for Oak {
    fn description(&self) -> &str {
        well_known_terms::descriptions::trees::OAK
    }
}

impl HasDisplayName for Oak {
    fn display_name(&self) -> &str {
        well_known_terms::trees::OAK
    }
}

impl ActorLike for Oak {}
