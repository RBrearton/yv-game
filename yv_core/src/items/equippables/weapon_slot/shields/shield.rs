use crate::prelude::*;

/// # Shield
/// An enum containing all shield slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Shield {
    WoodenBuckler(equippables::weapon_slot::shields::WoodenBuckler),
}
