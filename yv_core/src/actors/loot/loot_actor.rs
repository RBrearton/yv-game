use crate::prelude::*;

/// # Loot actor
/// The loot actor enum contains a variant for each type of directly lootable thing in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(ActorLike)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(Lootable)]
pub enum LootActor {
    LootBag(loot::LootBag),
}
