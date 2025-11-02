use crate::prelude::*;

/// # Shortsword
/// An enum containing all shortsword items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shortsword {
    StoneShortsword(equippables::weapon_slot::shortswords::StoneShortsword),
    RustyIronShortsword(equippables::weapon_slot::shortswords::RustyIronShortsword),
    IronShortsword(equippables::weapon_slot::shortswords::IronShortsword),
}

impl Shortsword {
    pub fn weapon_type(&self) -> WeaponType {
        match self {
            Shortsword::StoneShortsword(_) => WeaponType::Any,
            Shortsword::RustyIronShortsword(_) => WeaponType::Any,
            Shortsword::IronShortsword(_) => WeaponType::Any,
        }
    }
}
impl Describable for Shortsword {
    fn description(&self) -> &str {
        match self {
            Shortsword::StoneShortsword(stone_shortsword) => stone_shortsword.description(),
            Shortsword::RustyIronShortsword(rusty_iron_shortsword) => {
                rusty_iron_shortsword.description()
            }
            Shortsword::IronShortsword(iron_shortsword) => iron_shortsword.description(),
        }
    }
}

impl HasDisplayName for Shortsword {
    fn display_name(&self) -> &str {
        match self {
            Shortsword::StoneShortsword(stone_shortsword) => stone_shortsword.display_name(),
            Shortsword::RustyIronShortsword(rusty_iron_shortsword) => {
                rusty_iron_shortsword.display_name()
            }
            Shortsword::IronShortsword(iron_shortsword) => iron_shortsword.display_name(),
        }
    }
}

impl HasStats for Shortsword {
    fn stats(&self) -> Stats {
        match self {
            Shortsword::StoneShortsword(stone_shortsword) => stone_shortsword.stats(),
            Shortsword::RustyIronShortsword(rusty_iron_shortsword) => rusty_iron_shortsword.stats(),
            Shortsword::IronShortsword(iron_shortsword) => iron_shortsword.stats(),
        }
    }
}
