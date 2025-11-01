use crate::prelude::*;

const IRON_DAGGER_DESCRIPTION: &str = "A simple iron dagger.";
const IRON_DAGGER_DISPLAY_NAME: &str = "Iron dagger";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronDagger {
    pub stats: Stats,
}

impl Default for IronDagger {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 90),
                attack_power: Stat::new(StatType::AttackPower, 6),
                ..Stats::default()
            },
        }
    }
}

impl Describable for IronDagger {
    fn description(&self) -> &str {
        IRON_DAGGER_DESCRIPTION
    }
}

impl HasDisplayName for IronDagger {
    fn display_name(&self) -> &str {
        IRON_DAGGER_DISPLAY_NAME
    }
}

impl HasStats for IronDagger {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
