use crate::prelude::*;

/// # Greatsword
/// An enum containing all greatsword slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Greatsword {
    IronGreatsword(equippables::weapon_slot::greatswords::IronGreatsword),
}
