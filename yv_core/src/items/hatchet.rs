use crate::prelude::*;

/// # Hatchet
/// An enum containing all hatchet slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Hatchet {
    StoneHatchet(hatchets::StoneHatchet),
    IronHatchet(hatchets::IronHatchet),
}

impl Describable for Hatchet {
    fn description(&self) -> &str {
        match self {
            Hatchet::StoneHatchet(stone_hatchet) => stone_hatchet.description(),
            Hatchet::IronHatchet(iron_hatchet) => iron_hatchet.description(),
        }
    }
}

impl HasDisplayName for Hatchet {
    fn display_name(&self) -> &str {
        match self {
            Hatchet::StoneHatchet(stone_hatchet) => stone_hatchet.display_name(),
            Hatchet::IronHatchet(iron_hatchet) => iron_hatchet.display_name(),
        }
    }
}

impl HasStats for Hatchet {
    fn stats(&self) -> &Stats {
        match self {
            Hatchet::StoneHatchet(stone_hatchet) => stone_hatchet.stats(),
            Hatchet::IronHatchet(iron_hatchet) => iron_hatchet.stats(),
        }
    }
}
