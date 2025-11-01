use crate::prelude::*;

const IRON_PICKAXE_DESCRIPTION: &str = "A simple iron pickaxe.";
const IRON_PICKAXE_DISPLAY_NAME: &str = "Iron pickaxe";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronPickaxe {
    pub stats: Stats,
}

impl Default for IronPickaxe {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 45),
                attack_power: Stat::new(StatType::AttackPower, 6),
                mining_power: Stat::new(StatType::MiningPower, 4),
                ..Stats::default()
            },
        }
    }
}

impl Describable for IronPickaxe {
    fn description(&self) -> &str {
        IRON_PICKAXE_DESCRIPTION
    }
}

impl HasDisplayName for IronPickaxe {
    fn display_name(&self) -> &str {
        IRON_PICKAXE_DISPLAY_NAME
    }
}

impl HasStats for IronPickaxe {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
