use crate::prelude::*;

/// # Chest
/// An enum containing all chest slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Chest {
    LinenTunic(equippables::chest_slot::LinenTunic),
    LeatherTunic(equippables::chest_slot::LeatherTunic),
}
