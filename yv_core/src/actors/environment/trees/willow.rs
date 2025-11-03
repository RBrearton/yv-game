use crate::prelude::*;

/// # Willow
/// A willow tree is a type of tree that can be chopped down.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
pub struct Willow {
    pub core: ActorCore,
}

impl Describable for Willow {
    fn description(&self) -> &str {
        well_known_terms::descriptions::trees::WILLOW
    }
}

impl HasDisplayName for Willow {
    fn display_name(&self) -> &str {
        well_known_terms::trees::WILLOW
    }
}

impl ActorLike for Willow {}

impl Choppable for Willow {
    fn woodcutting_difficulty(&self) -> u16 {
        4
    }

    fn minimum_woodcutting_level(&self) -> u16 {
        3
    }

    fn wood(&self) -> wood::Log {
        wood::Log::Willow(wood::WillowLog::default())
    }
}
