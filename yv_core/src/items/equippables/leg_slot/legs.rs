use crate::prelude::*;

/// # Legs
/// An enum containing all legs slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Legs {
    LinenTrousers(equippables::leg_slot::LinenTrousers),
}

impl Describable for Legs {
    fn description(&self) -> &str {
        match self {
            Legs::LinenTrousers(linen_trousers) => linen_trousers.description(),
        }
    }
}

impl HasDisplayName for Legs {
    fn display_name(&self) -> &str {
        match self {
            Legs::LinenTrousers(linen_trousers) => linen_trousers.display_name(),
        }
    }
}

impl HasStats for Legs {
    fn stats(&self) -> &Stats {
        match self {
            Legs::LinenTrousers(linen_trousers) => linen_trousers.stats(),
        }
    }
}
