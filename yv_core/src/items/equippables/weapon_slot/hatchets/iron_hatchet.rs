use crate::prelude::*;

const IRON_HATCHET_DESCRIPTION: &str = "A simple iron hatchet.";
const IRON_HATCHET_DISPLAY_NAME: &str = "Iron hatchet";
const IRON_HATCHET_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 60),
    attack_power: Stat::new(StatType::AttackPower, 6),
    woodcutting_power: Stat::new(StatType::WoodcuttingPower, 4),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronHatchet {
    pub core: equippables::EquippableCore,
}

impl Describable for IronHatchet {
    fn description(&self) -> &str {
        IRON_HATCHET_DESCRIPTION
    }
}

impl HasDisplayName for IronHatchet {
    fn display_name(&self) -> &str {
        IRON_HATCHET_DISPLAY_NAME
    }
}

impl HasStats for IronHatchet {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_HATCHET_DEFAULT_STATS])
    }
}
