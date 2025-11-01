use crate::prelude::*;

const IRON_BATTLEAXE_DESCRIPTION: &str = "A simple iron battle axe.";
const IRON_BATTLEAXE_DISPLAY_NAME: &str = "Iron battleaxe";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronBattleaxe {
    pub stats: Stats,
}

impl Default for IronBattleaxe {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 38),
                attack_power: Stat::new(StatType::AttackPower, 19),
                ..Stats::default()
            },
        }
    }
}

impl Describable for IronBattleaxe {
    fn description(&self) -> &str {
        IRON_BATTLEAXE_DESCRIPTION
    }
}

impl HasDisplayName for IronBattleaxe {
    fn display_name(&self) -> &str {
        IRON_BATTLEAXE_DISPLAY_NAME
    }
}

impl HasStats for IronBattleaxe {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
