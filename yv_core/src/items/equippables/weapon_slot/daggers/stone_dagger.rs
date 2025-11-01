use crate::prelude::*;

const STONE_DAGGER_DESCRIPTION: &str = "A simple stone dagger.";
const STONE_DAGGER_DISPLAY_NAME: &str = "Stone dagger";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneDagger {
    pub stats: Stats,
}

impl Default for StoneDagger {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 90),
                attack_power: Stat::new(StatType::AttackPower, 3),
                ..Stats::default()
            },
        }
    }
}

impl Describable for StoneDagger {
    fn description(&self) -> &str {
        STONE_DAGGER_DESCRIPTION
    }
}

impl HasDisplayName for StoneDagger {
    fn display_name(&self) -> &str {
        STONE_DAGGER_DISPLAY_NAME
    }
}

impl HasStats for StoneDagger {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
