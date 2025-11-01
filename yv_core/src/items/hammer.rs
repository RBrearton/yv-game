use crate::prelude::*;

/// # Hammer
/// An enum containing all hammer items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Hammer {
    WoodenClub(hammers::WoodenClub),
    IronMace(hammers::IronMace),
}

impl Describable for Hammer {
    fn description(&self) -> &str {
        match self {
            Hammer::WoodenClub(wooden_club) => wooden_club.description(),
            Hammer::IronMace(iron_mace) => iron_mace.description(),
        }
    }
}

impl HasDisplayName for Hammer {
    fn display_name(&self) -> &str {
        match self {
            Hammer::WoodenClub(wooden_club) => wooden_club.display_name(),
            Hammer::IronMace(iron_mace) => iron_mace.display_name(),
        }
    }
}

impl HasStats for Hammer {
    fn stats(&self) -> &Stats {
        match self {
            Hammer::WoodenClub(wooden_club) => wooden_club.stats(),
            Hammer::IronMace(iron_mace) => iron_mace.stats(),
        }
    }
}
