use crate::prelude::*;

const STONE_SHORTSWORD_DESCRIPTION: &str = "A crude stone imitation of a shortsword.";
const STONE_SHORTSWORD_DISPLAY_NAME: &str = "Stone shortsword";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneShortsword {
    pub stats: Stats,
}

impl Default for StoneShortsword {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 75),
                attack_power: Stat::new(StatType::AttackPower, 5),
                ..Stats::default()
            },
        }
    }
}

impl Describable for StoneShortsword {
    fn description(&self) -> &str {
        STONE_SHORTSWORD_DESCRIPTION
    }
}

impl HasDisplayName for StoneShortsword {
    fn display_name(&self) -> &str {
        STONE_SHORTSWORD_DISPLAY_NAME
    }
}

impl HasStats for StoneShortsword {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
