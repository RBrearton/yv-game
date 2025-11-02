use crate::prelude::*;

/// # Legs
/// An enum containing all legs slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Legs {
    LinenTrousers(equippables::leg_slot::LinenTrousers),
}
