use crate::prelude::*;

const IRON_MACE_DESCRIPTION: &str = "A simple iron mace.";
const IRON_MACE_DISPLAY_NAME: &str = "Iron mace";
const IRON_MACE_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 60),
    attack_power: Stat::new(StatType::AttackPower, 12),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronMace {
    pub core: equippables::EquippableCore,
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
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_MACE_DEFAULT_STATS])
    }
}
