use crate::prelude::*;

const STONE_HATCHET_DESCRIPTION: &str = "A simple stone hatchet.";
const STONE_HATCHET_DISPLAY_NAME: &str = "Stone hatchet";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneHatchet {
    pub stats: Stats,
}

impl Default for StoneHatchet {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 60),
                attack_power: Stat::new(StatType::AttackPower, 4),
                woodcutting_power: Stat::new(StatType::WoodcuttingPower, 2),
                ..Stats::default()
            },
        }
    }
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
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
