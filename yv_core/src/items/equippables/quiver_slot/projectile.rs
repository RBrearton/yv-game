use crate::prelude::*;

/// # Projectile
/// An enum containing all projectile items in the game (really, these are just things that go in
/// a quiver).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Projectile {
    StoneArrow(equippables::quiver_slot::StoneArrow),
    IronArrow(equippables::quiver_slot::IronArrow),
    SteelArrow(equippables::quiver_slot::SteelArrow),
}

impl Describable for Projectile {
    fn description(&self) -> &str {
        match self {
            Projectile::StoneArrow(stone_arrow) => stone_arrow.description(),
            Projectile::IronArrow(iron_arrow) => iron_arrow.description(),
            Projectile::SteelArrow(steel_arrow) => steel_arrow.description(),
        }
    }
}

impl HasDisplayName for Projectile {
    fn display_name(&self) -> &str {
        match self {
            Projectile::StoneArrow(stone_arrow) => stone_arrow.display_name(),
            Projectile::IronArrow(iron_arrow) => iron_arrow.display_name(),
            Projectile::SteelArrow(steel_arrow) => steel_arrow.display_name(),
        }
    }
}

impl HasStats for Projectile {
    fn stats(&self) -> Stats {
        match self {
            Projectile::StoneArrow(stone_arrow) => stone_arrow.stats(),
            Projectile::IronArrow(iron_arrow) => iron_arrow.stats(),
            Projectile::SteelArrow(steel_arrow) => steel_arrow.stats(),
        }
    }
}
