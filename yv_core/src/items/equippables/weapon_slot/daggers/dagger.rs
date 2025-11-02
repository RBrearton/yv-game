use crate::prelude::*;

/// # Dagger
/// An enum containing all dagger slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Dagger {
    WoodenDagger(equippables::weapon_slot::daggers::WoodenDagger),
    StoneDagger(equippables::weapon_slot::daggers::StoneDagger),
    IronDagger(equippables::weapon_slot::daggers::IronDagger),
}

impl Describable for Dagger {
    fn description(&self) -> &str {
        match self {
            Dagger::WoodenDagger(wooden_dagger) => wooden_dagger.description(),
            Dagger::StoneDagger(stone_dagger) => stone_dagger.description(),
            Dagger::IronDagger(iron_dagger) => iron_dagger.description(),
        }
    }
}

impl HasDisplayName for Dagger {
    fn display_name(&self) -> &str {
        match self {
            Dagger::WoodenDagger(wooden_dagger) => wooden_dagger.display_name(),
            Dagger::StoneDagger(stone_dagger) => stone_dagger.display_name(),
            Dagger::IronDagger(iron_dagger) => iron_dagger.display_name(),
        }
    }
}

impl HasStats for Dagger {
    fn stats(&self) -> Stats {
        match self {
            Dagger::WoodenDagger(wooden_dagger) => wooden_dagger.stats(),
            Dagger::StoneDagger(stone_dagger) => stone_dagger.stats(),
            Dagger::IronDagger(iron_dagger) => iron_dagger.stats(),
        }
    }
}
