use crate::prelude::*;

/// # Enchantment
/// An enchantment is a way of magically altering/enhancing an equippable item.
/// These are upgrades to an item that can be applied by experts in the magical arts.
/// These upgrades are very powerful, but tend to be temporary.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Enchantment {
    Haste,
    MinorHarvesting,
}

impl HasDisplayName for Enchantment {
    fn display_name(&self) -> &str {
        match self {
            Enchantment::Haste => well_known_terms::enchantments::HASTE,
            Enchantment::MinorHarvesting => well_known_terms::enchantments::MINOR_HARVESTING,
        }
    }
}

impl Describable for Enchantment {
    fn description(&self) -> &str {
        match self {
            Enchantment::Haste => well_known_terms::descriptions::enchantments::HASTE,
            Enchantment::MinorHarvesting => {
                well_known_terms::descriptions::enchantments::MINOR_HARVESTING
            }
        }
    }
}

impl HasStats for Enchantment {
    fn stats(&self) -> Stats {
        match self {
            Enchantment::Haste => Stats {
                speed: Stat::new(StatType::Speed, 15),
                ..Stats::default()
            },
            Enchantment::MinorHarvesting => Stats {
                mining_power: Stat::new(StatType::MiningPower, 7),
                woodcutting_power: Stat::new(StatType::WoodcuttingPower, 7),
                ..Stats::default()
            },
        }
    }
}
