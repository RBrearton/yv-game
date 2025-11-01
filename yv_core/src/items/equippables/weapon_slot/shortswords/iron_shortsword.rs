use crate::prelude::*;

const IRON_SHORTSWORD_DESCRIPTION: &str = "A simple iron shortsword.";
const IRON_SHORTSWORD_DISPLAY_NAME: &str = "Iron shortsword";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronShortsword {
    pub stats: Stats,
}

impl Default for IronShortsword {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 75),
                attack_power: Stat::new(StatType::AttackPower, 13),
                ..Stats::default()
            },
        }
    }
}

impl Describable for IronShortsword {
    fn description(&self) -> &str {
        IRON_SHORTSWORD_DESCRIPTION
    }
}

impl HasDisplayName for IronShortsword {
    fn display_name(&self) -> &str {
        IRON_SHORTSWORD_DISPLAY_NAME
    }
}

impl HasStats for IronShortsword {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
