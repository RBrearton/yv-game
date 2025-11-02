use crate::prelude::*;

/// # Hatchet
/// An enum containing all hatchet slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Hatchet {
    StoneHatchet(equippables::weapon_slot::hatchets::StoneHatchet),
    IronHatchet(equippables::weapon_slot::hatchets::IronHatchet),
}
