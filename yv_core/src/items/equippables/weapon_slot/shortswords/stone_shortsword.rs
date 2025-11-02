use crate::prelude::*;

const STONE_SHORTSWORD_DESCRIPTION: &str = "A crude stone imitation of a shortsword.";
const STONE_SHORTSWORD_DISPLAY_NAME: &str = "Stone shortsword";
const STONE_SHORTSWORD_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 75),
    attack_power: Stat::new(StatType::AttackPower, 5),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneShortsword {
    pub augmentation: Option<Augmentation>,
    pub durability: Durability,
}

impl Describable for StoneShortsword {
    fn description(&self) -> &str {
        STONE_SHORTSWORD_DESCRIPTION
    }
}

impl HasDisplayName for StoneShortsword {
    fn display_name(&self) -> &str {
        STONE_SHORTSWORD_DISPLAY_NAME
    }
}

impl HasStats for StoneShortsword {
    fn stats(&self) -> Stats {
        let augmentation_stats = self.augmentation.map(|augmentation| augmentation.stats());
        let stats = Stats::add([
            augmentation_stats.unwrap_or_default(),
            STONE_SHORTSWORD_DEFAULT_STATS,
        ]);
        stats
    }
}
