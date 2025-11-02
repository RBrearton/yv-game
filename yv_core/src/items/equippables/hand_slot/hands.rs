use crate::prelude::*;

/// # Hands
/// An enum containing all hands slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Hands {
    WoolGloves(equippables::hand_slot::WoolGloves),
}
