use crate::prelude::*;

/// # Finger
/// An enum containing all finger slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Finger {
    GoldRing(equippables::finger_slot::GoldRing),
}
