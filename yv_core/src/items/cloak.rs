use crate::prelude::*;

/// # Cloak
/// An enum containing all cloak slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Cloak {
    RaggedCloak(cloaks::RaggedCloak),
}

impl Describable for Cloak {
    fn description(&self) -> &str {
        match self {
            Cloak::RaggedCloak(ragged_cloak) => ragged_cloak.description(),
        }
    }
}

impl HasDisplayName for Cloak {
    fn display_name(&self) -> &str {
        match self {
            Cloak::RaggedCloak(ragged_cloak) => ragged_cloak.display_name(),
        }
    }
}

impl HasStats for Cloak {
    fn stats(&self) -> &Stats {
        match self {
            Cloak::RaggedCloak(ragged_cloak) => ragged_cloak.stats(),
        }
    }
}
