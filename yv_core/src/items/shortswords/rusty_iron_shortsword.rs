use crate::prelude::*;

const RUSTY_IRON_SHORTSWORD_DESCRIPTION: &str = "An iron shortsword that has lost its edge.";
const RUSTY_IRON_SHORTSWORD_DISPLAY_NAME: &str = "Rusty iron shortsword";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RustyIronShortsword {
    pub stats: Stats,
}

impl Default for RustyIronShortsword {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 75),
                attack_power: Stat::new(StatType::AttackPower, 7),
                ..Stats::default()
            },
        }
    }
}

impl Describable for RustyIronShortsword {
    fn description(&self) -> &str {
        RUSTY_IRON_SHORTSWORD_DESCRIPTION
    }
}

impl HasDisplayName for RustyIronShortsword {
    fn display_name(&self) -> &str {
        RUSTY_IRON_SHORTSWORD_DISPLAY_NAME
    }
}

impl HasStats for RustyIronShortsword {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
