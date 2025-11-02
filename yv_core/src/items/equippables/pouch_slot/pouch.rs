use crate::prelude::*;

/// # Pouch
/// An enum containing all pouch slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Pouch {
    StoneIdol(equippables::pouch_slot::StoneIdol),
}
