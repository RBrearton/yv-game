use crate::prelude::*;

/// # Dagger
/// An enum containing all dagger slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Dagger {
    WoodenDagger(equippables::weapon_slot::daggers::WoodenDagger),
    StoneDagger(equippables::weapon_slot::daggers::StoneDagger),
    IronDagger(equippables::weapon_slot::daggers::IronDagger),
}
