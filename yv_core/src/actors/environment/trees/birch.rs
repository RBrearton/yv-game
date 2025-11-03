use crate::prelude::*;

/// # Birch
/// A birch tree is a type of tree that can be chopped down.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
pub struct Birch {
    pub core: ActorCore,
}

impl Describable for Birch {
    fn description(&self) -> &str {
        well_known_terms::descriptions::trees::BIRCH
    }
}

impl HasDisplayName for Birch {
    fn display_name(&self) -> &str {
        well_known_terms::trees::BIRCH
    }
}

impl ActorLike for Birch {}

impl Choppable for Birch {
    fn woodcutting_difficulty(&self) -> u16 {
        7
    }

    fn minimum_woodcutting_level(&self) -> u16 {
        8
    }

    fn wood(&self) -> wood::Log {
        wood::Log::Birch(wood::BirchLog::default())
    }
}
