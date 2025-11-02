use crate::prelude::*;

/// # Cloak
/// An enum containing all cloak slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Cloak {
    RaggedCloak(equippables::back_slot::RaggedCloak),
}
