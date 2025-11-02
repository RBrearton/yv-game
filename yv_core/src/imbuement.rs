use crate::prelude::*;

/// # Imbuement
/// An imbuement is a way of spiritually altering/enhancing an equippable item.
/// These are upgrades to an item that clerics can perform, but only to their own equipment.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Imbuement {
    Vitality,
    Warmth,
}

impl HasDisplayName for Imbuement {
    fn display_name(&self) -> &str {
        match self {
            Imbuement::Vitality => well_known_terms::imbuements::VITALITY,
            Imbuement::Warmth => well_known_terms::imbuements::WARMTH,
        }
    }
}

impl Describable for Imbuement {
    fn description(&self) -> &str {
        match self {
            Imbuement::Vitality => well_known_terms::descriptions::imbuements::VITALITY,
            Imbuement::Warmth => well_known_terms::descriptions::imbuements::WARMTH,
        }
    }
}

impl HasStats for Imbuement {
    fn stats(&self) -> Stats {
        match self {
            Imbuement::Vitality => Stats {
                vitality: Stat::new(StatType::Vitality, 4),
                ..Stats::default()
            },
            Imbuement::Warmth => Stats {
                warmth: Stat::new(StatType::Warmth, 3),
                ..Stats::default()
            },
        }
    }
}
