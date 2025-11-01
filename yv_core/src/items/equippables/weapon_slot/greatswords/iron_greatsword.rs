use crate::prelude::*;

const IRON_GREATSWORD_DESCRIPTION: &str = "A large iron greatsword.";
const IRON_GREATSWORD_DISPLAY_NAME: &str = "Iron greatsword";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronGreatsword {
    pub stats: Stats,
}

impl Default for IronGreatsword {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 30),
                attack_power: Stat::new(StatType::AttackPower, 35),
                ..Stats::default()
            },
        }
    }
}

impl Describable for IronGreatsword {
    fn description(&self) -> &str {
        IRON_GREATSWORD_DESCRIPTION
    }
}

impl HasDisplayName for IronGreatsword {
    fn display_name(&self) -> &str {
        IRON_GREATSWORD_DISPLAY_NAME
    }
}

impl HasStats for IronGreatsword {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
