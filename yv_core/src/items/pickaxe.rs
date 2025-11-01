use crate::prelude::*;

/// # Pickaxe
/// An enum containing all pickaxe slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Pickaxe {
    StonePickaxe(pickaxes::StonePickaxe),
    IronPickaxe(pickaxes::IronPickaxe),
}

impl Describable for Pickaxe {
    fn description(&self) -> &str {
        match self {
            Pickaxe::StonePickaxe(stone_pickaxe) => stone_pickaxe.description(),
            Pickaxe::IronPickaxe(iron_pickaxe) => iron_pickaxe.description(),
        }
    }
}

impl HasDisplayName for Pickaxe {
    fn display_name(&self) -> &str {
        match self {
            Pickaxe::StonePickaxe(stone_pickaxe) => stone_pickaxe.display_name(),
            Pickaxe::IronPickaxe(iron_pickaxe) => iron_pickaxe.display_name(),
        }
    }
}

impl HasStats for Pickaxe {
    fn stats(&self) -> &Stats {
        match self {
            Pickaxe::StonePickaxe(stone_pickaxe) => stone_pickaxe.stats(),
            Pickaxe::IronPickaxe(iron_pickaxe) => iron_pickaxe.stats(),
        }
    }
}
