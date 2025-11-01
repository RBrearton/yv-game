use crate::prelude::*;

const WOODEN_CLUB_DESCRIPTION: &str = "A heavy wooden club.";
const WOODEN_CLUB_DISPLAY_NAME: &str = "Wooden club";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoodenClub {
    pub stats: Stats,
}

impl Default for WoodenClub {
    fn default() -> Self {
        Self {
            stats: Stats {
                weapon_speed: Stat::new(StatType::WeaponSpeed, 54),
                attack_power: Stat::new(StatType::AttackPower, 11),
                ..Stats::default()
            },
        }
    }
}

impl Describable for WoodenClub {
    fn description(&self) -> &str {
        WOODEN_CLUB_DESCRIPTION
    }
}

impl HasDisplayName for WoodenClub {
    fn display_name(&self) -> &str {
        WOODEN_CLUB_DISPLAY_NAME
    }
}

impl HasStats for WoodenClub {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
