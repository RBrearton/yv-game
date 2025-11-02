use crate::prelude::*;

const STONE_DAGGER_DESCRIPTION: &str = "A simple stone dagger.";
const STONE_DAGGER_DISPLAY_NAME: &str = "Stone dagger";
const STONE_DAGGER_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 90),
    attack_power: Stat::new(StatType::AttackPower, 3),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneDagger {
    pub core: equippables::EquippableCore,
}

impl Describable for StoneDagger {
    fn description(&self) -> &str {
        STONE_DAGGER_DESCRIPTION
    }
}

impl HasDisplayName for StoneDagger {
    fn display_name(&self) -> &str {
        STONE_DAGGER_DISPLAY_NAME
    }
}

impl HasStats for StoneDagger {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, STONE_DAGGER_DEFAULT_STATS])
    }
}
