use crate::prelude::*;

/// # Hammer
/// An enum containing all hammer items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Hammer {
    WoodenClub(equippables::weapon_slot::hammers::WoodenClub),
    IronMace(equippables::weapon_slot::hammers::IronMace),
}

impl Hammer {
    pub fn weapon_type(&self) -> WeaponType {
        match self {
            Hammer::WoodenClub(_) => WeaponType::Any,
            Hammer::IronMace(_) => WeaponType::Any,
        }
    }
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
    fn stats(&self) -> Stats {
        match self {
            Hammer::WoodenClub(wooden_club) => wooden_club.stats(),
            Hammer::IronMace(iron_mace) => iron_mace.stats(),
        }
    }
}

impl Durable for Hammer {
    fn durability(&self) -> &Durability {
        match self {
            Hammer::WoodenClub(wooden_club) => wooden_club.durability(),
            Hammer::IronMace(iron_mace) => iron_mace.durability(),
        }
    }

    fn durability_mut(&mut self) -> &mut Durability {
        match self {
            Hammer::WoodenClub(wooden_club) => wooden_club.durability_mut(),
            Hammer::IronMace(iron_mace) => iron_mace.durability_mut(),
        }
    }
}
