use crate::prelude::*;

/// # Battleaxe
/// An enum containing all battleaxe slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Battleaxe {
    StoneBattleaxe(equippables::weapon_slot::battleaxes::StoneBattleaxe),
    RustyIronBattleaxe(equippables::weapon_slot::battleaxes::RustyIronBattleaxe),
    IronBattleaxe(equippables::weapon_slot::battleaxes::IronBattleaxe),
}
