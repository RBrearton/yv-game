use crate::prelude::*;

/// # Battleaxe
/// An enum containing all battleaxe slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Battleaxe {
    StoneBattleaxe(equippables::weapon_slot::battleaxes::StoneBattleaxe),
    RustyIronBattleaxe(equippables::weapon_slot::battleaxes::RustyIronBattleaxe),
    IronBattleaxe(equippables::weapon_slot::battleaxes::IronBattleaxe),
}

impl Describable for Battleaxe {
    fn description(&self) -> &str {
        match self {
            Battleaxe::StoneBattleaxe(stone_battleaxe) => stone_battleaxe.description(),
            Battleaxe::RustyIronBattleaxe(rusty_iron_battleaxe) => {
                rusty_iron_battleaxe.description()
            }
            Battleaxe::IronBattleaxe(iron_battleaxe) => iron_battleaxe.description(),
        }
    }
}

impl HasDisplayName for Battleaxe {
    fn display_name(&self) -> &str {
        match self {
            Battleaxe::StoneBattleaxe(stone_battleaxe) => stone_battleaxe.display_name(),
            Battleaxe::RustyIronBattleaxe(rusty_iron_battleaxe) => {
                rusty_iron_battleaxe.display_name()
            }
            Battleaxe::IronBattleaxe(iron_battleaxe) => iron_battleaxe.display_name(),
        }
    }
}

impl HasStats for Battleaxe {
    fn stats(&self) -> Stats {
        match self {
            Battleaxe::StoneBattleaxe(stone_battleaxe) => stone_battleaxe.stats(),
            Battleaxe::RustyIronBattleaxe(rusty_iron_battleaxe) => rusty_iron_battleaxe.stats(),
            Battleaxe::IronBattleaxe(iron_battleaxe) => iron_battleaxe.stats(),
        }
    }
}

impl Durable for Battleaxe {
    fn durability(&self) -> &Durability {
        match self {
            Battleaxe::StoneBattleaxe(stone_battleaxe) => stone_battleaxe.durability(),
            Battleaxe::RustyIronBattleaxe(rusty_iron_battleaxe) => rusty_iron_battleaxe.durability(),
            Battleaxe::IronBattleaxe(iron_battleaxe) => iron_battleaxe.durability(),
        }
    }

    fn durability_mut(&mut self) -> &mut Durability {
        match self {
            Battleaxe::StoneBattleaxe(stone_battleaxe) => stone_battleaxe.durability_mut(),
            Battleaxe::RustyIronBattleaxe(rusty_iron_battleaxe) => rusty_iron_battleaxe.durability_mut(),
            Battleaxe::IronBattleaxe(iron_battleaxe) => iron_battleaxe.durability_mut(),
        }
    }
}
