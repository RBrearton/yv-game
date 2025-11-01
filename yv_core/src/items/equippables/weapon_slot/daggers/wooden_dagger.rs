use crate::prelude::*;

const WOODEN_DAGGER_DESCRIPTION: &str = "A simple wooden dagger.";
const WOODEN_DAGGER_DISPLAY_NAME: &str = "Wooden dagger";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoodenDagger {
    pub stats: Stats,
}

impl Default for WoodenDagger {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 90),
                attack_power: Stat::new(StatType::AttackPower, 1),
                ..Stats::default()
            },
        }
    }
}

impl Describable for WoodenDagger {
    fn description(&self) -> &str {
        WOODEN_DAGGER_DESCRIPTION
    }
}

impl HasDisplayName for WoodenDagger {
    fn display_name(&self) -> &str {
        WOODEN_DAGGER_DISPLAY_NAME
    }
}

impl HasStats for WoodenDagger {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
