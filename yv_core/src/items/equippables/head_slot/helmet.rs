use crate::prelude::*;

/// # Helmet
/// An enum containing all helmet slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Helmet {
    IronSkullcap(equippables::head_slot::IronSkullcap),
}
