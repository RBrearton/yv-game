use crate::prelude::*;

const STONE_PICKAXE_DESCRIPTION: &str = "A very basic stone pickaxe.";
const STONE_PICKAXE_DISPLAY_NAME: &str = "Stone pickaxe";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StonePickaxe {
    pub stats: Stats,
}

impl Default for StonePickaxe {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 45),
                attack_power: Stat::new(StatType::AttackPower, 5),
                mining_power: Stat::new(StatType::MiningPower, 2),
                ..Stats::default()
            },
        }
    }
}

impl Describable for StonePickaxe {
    fn description(&self) -> &str {
        STONE_PICKAXE_DESCRIPTION
    }
}

impl HasDisplayName for StonePickaxe {
    fn display_name(&self) -> &str {
        STONE_PICKAXE_DISPLAY_NAME
    }
}

impl HasStats for StonePickaxe {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
