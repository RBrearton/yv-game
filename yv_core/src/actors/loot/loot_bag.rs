use crate::prelude::*;

/// # Loot bag
/// A loot bag is a special kind of actor that is used to store loot.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(Lootable)]
pub struct LootBag {
    pub lootable_core: loot::LootableCore,
}

impl Describable for LootBag {
    fn description(&self) -> &str {
        well_known_terms::descriptions::loot::LOOT_BAG
    }
}

impl HasDisplayName for LootBag {
    fn display_name(&self) -> &str {
        well_known_terms::loot::LOOT_BAG
    }
}

impl ActorLike for LootBag {}
