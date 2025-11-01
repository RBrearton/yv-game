use crate::prelude::*;

const RUSTY_IRON_BATTLEAXE_DESCRIPTION: &str = "An iron battle axe that has seen better days.";
const RUSTY_IRON_BATTLEAXE_DISPLAY_NAME: &str = "Rusty iron battleaxe";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RustyIronBattleaxe {
    pub stats: Stats,
}

impl Default for RustyIronBattleaxe {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 38),
                attack_power: Stat::new(StatType::AttackPower, 14),
                ..Stats::default()
            },
        }
    }
}

impl Describable for RustyIronBattleaxe {
    fn description(&self) -> &str {
        RUSTY_IRON_BATTLEAXE_DESCRIPTION
    }
}

impl HasDisplayName for RustyIronBattleaxe {
    fn display_name(&self) -> &str {
        RUSTY_IRON_BATTLEAXE_DISPLAY_NAME
    }
}

impl HasStats for RustyIronBattleaxe {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
