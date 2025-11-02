use crate::prelude::*;

/// # Hammer
/// An enum containing all hammer items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Hammer {
    WoodenClub(equippables::weapon_slot::hammers::WoodenClub),
    IronMace(equippables::weapon_slot::hammers::IronMace),
}
