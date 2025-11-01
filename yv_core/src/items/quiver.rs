use crate::prelude::*;

/// # Quiver
/// An enum containing all quiver slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Quiver {
    StoneArrow(arrows::StoneArrow),
    IronArrow(arrows::IronArrow),
    SteelArrow(arrows::SteelArrow),
}

impl Describable for Quiver {
    fn description(&self) -> &str {
        match self {
            Quiver::StoneArrow(stone_arrow) => stone_arrow.description(),
            Quiver::IronArrow(iron_arrow) => iron_arrow.description(),
            Quiver::SteelArrow(steel_arrow) => steel_arrow.description(),
        }
    }
}

impl HasDisplayName for Quiver {
    fn display_name(&self) -> &str {
        match self {
            Quiver::StoneArrow(stone_arrow) => stone_arrow.display_name(),
            Quiver::IronArrow(iron_arrow) => iron_arrow.display_name(),
            Quiver::SteelArrow(steel_arrow) => steel_arrow.display_name(),
        }
    }
}

impl HasStats for Quiver {
    fn stats(&self) -> &Stats {
        match self {
            Quiver::StoneArrow(stone_arrow) => stone_arrow.stats(),
            Quiver::IronArrow(iron_arrow) => iron_arrow.stats(),
            Quiver::SteelArrow(steel_arrow) => steel_arrow.stats(),
        }
    }
}
