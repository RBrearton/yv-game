use crate::prelude::*;

/// # Augmentation
/// An augmentation is a way of physically altering/enhancing an equippable item.
/// These are upgrades to an item that can be applied by expert blacksmiths, tailors,
/// leatherworkers etc.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Augmentation {
    Sharpened,
    LightReinforcement,
    Strengthened,
}

impl HasDisplayName for Augmentation {
    fn display_name(&self) -> &str {
        match self {
            Augmentation::Sharpened => well_known_terms::augmentations::SHARPENED,
            Augmentation::LightReinforcement => {
                well_known_terms::augmentations::LIGHT_REINFORCEMENT
            }
            Augmentation::Strengthened => well_known_terms::augmentations::STRENGTHENED,
        }
    }
}

impl Describable for Augmentation {
    fn description(&self) -> &str {
        match self {
            Augmentation::Sharpened => well_known_terms::augmentations::SHARPENED,
            Augmentation::LightReinforcement => {
                well_known_terms::augmentations::LIGHT_REINFORCEMENT
            }
            Augmentation::Strengthened => well_known_terms::augmentations::STRENGTHENED,
        }
    }
}

impl HasStats for Augmentation {
    fn stats(&self) -> Stats {
        match self {
            Augmentation::Sharpened => Stats {
                slashing_bonus: Stat::new(StatType::SlashingBonus, 1),
                stabbing_bonus: Stat::new(StatType::StabbingBonus, 1),
                ..Stats::default()
            },
            Augmentation::LightReinforcement => Stats {
                armour: Stat::new(StatType::Armour, 2),
                speed: Stat::new(StatType::Speed, -1),
                ..Stats::default()
            },
            Augmentation::Strengthened => Stats {
                durability: Stat::new(StatType::Durability, 3),
                speed: Stat::new(StatType::Speed, -1),
                ..Stats::default()
            },
        }
    }
}
