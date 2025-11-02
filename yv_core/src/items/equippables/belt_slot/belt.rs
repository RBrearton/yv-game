use crate::prelude::*;

/// # Belt
/// An enum containing all belt slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Belt {
    SimpleLeatherBelt(equippables::belt_slot::SimpleLeatherBelt),
}
