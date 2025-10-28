use crate::prelude::*;

/// # Quest
/// An enum with a value for each quest in the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quest {
    Tutorial,
    ExampleQuest,
}

impl HasDisplayName for Quest {
    fn display_name(&self) -> &str {
        match self {
            Quest::Tutorial => well_known_terms::quests::TUTORIAL,
            Quest::ExampleQuest => well_known_terms::quests::EXAMPLE_QUEST,
        }
    }
}

impl Describable for Quest {
    fn description(&self) -> &str {
        match self {
            Quest::Tutorial => well_known_terms::descriptions::quests::TUTORIAL,
            Quest::ExampleQuest => well_known_terms::descriptions::quests::EXAMPLE_QUEST,
        }
    }
}
