use crate::prelude::*;

/// # Pickaxe
/// An enum containing all pickaxe slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Pickaxe {
    StonePickaxe(equippables::weapon_slot::pickaxes::StonePickaxe),
    IronPickaxe(equippables::weapon_slot::pickaxes::IronPickaxe),
}
