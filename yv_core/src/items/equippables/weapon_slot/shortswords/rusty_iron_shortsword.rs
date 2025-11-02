use crate::prelude::*;

const RUSTY_IRON_SHORTSWORD_DESCRIPTION: &str = "An iron shortsword that has lost its edge.";
const RUSTY_IRON_SHORTSWORD_DISPLAY_NAME: &str = "Rusty iron shortsword";
const RUSTY_IRON_SHORTSWORD_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 75),
    attack_power: Stat::new(StatType::AttackPower, 7),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RustyIronShortsword {
    pub augmentation: Option<Augmentation>,
    pub durability: Durability,
}

impl Describable for RustyIronShortsword {
    fn description(&self) -> &str {
        RUSTY_IRON_SHORTSWORD_DESCRIPTION
    }
}

impl HasDisplayName for RustyIronShortsword {
    fn display_name(&self) -> &str {
        RUSTY_IRON_SHORTSWORD_DISPLAY_NAME
    }
}

impl HasStats for RustyIronShortsword {
    fn stats(&self) -> Stats {
        let augmentation_stats = self.augmentation.map(|augmentation| augmentation.stats());
        let stats = Stats::add([
            augmentation_stats.unwrap_or_default(),
            RUSTY_IRON_SHORTSWORD_DEFAULT_STATS,
        ]);
        stats
    }
}
