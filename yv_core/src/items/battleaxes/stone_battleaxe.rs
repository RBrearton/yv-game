use crate::prelude::*;

const STONE_BATTLEAXE_DESCRIPTION: &str = "A crude stone battle axe.";
const STONE_BATTLEAXE_DISPLAY_NAME: &str = "Stone battleaxe";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneBattleaxe {
    pub stats: Stats,
}

impl Default for StoneBattleaxe {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 38),
                attack_power: Stat::new(StatType::AttackPower, 12),
                ..Stats::default()
            },
        }
    }
}

impl Describable for StoneBattleaxe {
    fn description(&self) -> &str {
        STONE_BATTLEAXE_DESCRIPTION
    }
}

impl HasDisplayName for StoneBattleaxe {
    fn display_name(&self) -> &str {
        STONE_BATTLEAXE_DISPLAY_NAME
    }
}

impl HasStats for StoneBattleaxe {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
