use crate::prelude::*;

/// # Wrists
/// An enum containing all wrist slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Wrists {
    LeatherBracer(bracers::LeatherBracer),
}

impl Describable for Wrists {
    fn description(&self) -> &str {
        match self {
            Wrists::LeatherBracer(leather_bracer) => leather_bracer.description(),
        }
    }
}

impl HasDisplayName for Wrists {
    fn display_name(&self) -> &str {
        match self {
            Wrists::LeatherBracer(leather_bracer) => leather_bracer.display_name(),
        }
    }
}

impl HasStats for Wrists {
    fn stats(&self) -> &Stats {
        match self {
            Wrists::LeatherBracer(leather_bracer) => leather_bracer.stats(),
        }
    }
}
