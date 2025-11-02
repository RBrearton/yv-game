use crate::prelude::*;

/// # Footwear
/// An enum containing all feet slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Footwear {
    OldLeatherSandals(equippables::foot_slot::OldLeatherSandals),
}
