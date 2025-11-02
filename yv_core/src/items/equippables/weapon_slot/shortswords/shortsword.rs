use crate::prelude::*;

/// # Shortsword
/// An enum containing all shortsword items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Shortsword {
    StoneShortsword(equippables::weapon_slot::shortswords::StoneShortsword),
    RustyIronShortsword(equippables::weapon_slot::shortswords::RustyIronShortsword),
    IronShortsword(equippables::weapon_slot::shortswords::IronShortsword),
}
