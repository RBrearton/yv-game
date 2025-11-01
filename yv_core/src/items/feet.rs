use crate::prelude::*;

/// # Feet
/// An enum containing all feet slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Feet {
    OldLeatherSandals(footwear::OldLeatherSandals),
}

impl Describable for Feet {
    fn description(&self) -> &str {
        match self {
            Feet::OldLeatherSandals(old_leather_sandals) => old_leather_sandals.description(),
        }
    }
}

impl HasDisplayName for Feet {
    fn display_name(&self) -> &str {
        match self {
            Feet::OldLeatherSandals(old_leather_sandals) => old_leather_sandals.display_name(),
        }
    }
}

impl HasStats for Feet {
    fn stats(&self) -> &Stats {
        match self {
            Feet::OldLeatherSandals(old_leather_sandals) => old_leather_sandals.stats(),
        }
    }
}
