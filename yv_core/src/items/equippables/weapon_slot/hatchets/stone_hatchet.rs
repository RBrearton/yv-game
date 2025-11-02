use crate::prelude::*;

const STONE_HATCHET_DESCRIPTION: &str = "A simple stone hatchet.";
const STONE_HATCHET_DISPLAY_NAME: &str = "Stone hatchet";
const STONE_HATCHET_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 60),
    attack_power: Stat::new(StatType::AttackPower, 4),
    woodcutting_power: Stat::new(StatType::WoodcuttingPower, 2),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneHatchet {
    pub core: equippables::EquippableCore,
}

impl Describable for StoneHatchet {
    fn description(&self) -> &str {
        STONE_HATCHET_DESCRIPTION
    }
}

impl HasDisplayName for StoneHatchet {
    fn display_name(&self) -> &str {
        STONE_HATCHET_DISPLAY_NAME
    }
}

impl HasStats for StoneHatchet {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, STONE_HATCHET_DEFAULT_STATS])
    }
}
