use crate::prelude::*;

const IRON_MACE_DESCRIPTION: &str = "A simple iron mace.";
const IRON_MACE_DISPLAY_NAME: &str = "Iron mace";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronMace {
    pub stats: Stats,
}

impl Default for IronMace {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 60),
                attack_power: Stat::new(StatType::AttackPower, 12),
                ..Stats::default()
            },
        }
    }
}

impl Describable for IronMace {
    fn description(&self) -> &str {
        IRON_MACE_DESCRIPTION
    }
}

impl HasDisplayName for IronMace {
    fn display_name(&self) -> &str {
        IRON_MACE_DISPLAY_NAME
    }
}

impl HasStats for IronMace {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
