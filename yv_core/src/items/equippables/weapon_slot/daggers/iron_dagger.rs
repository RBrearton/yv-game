use crate::prelude::*;

const IRON_DAGGER_DESCRIPTION: &str = "A simple iron dagger.";
const IRON_DAGGER_DISPLAY_NAME: &str = "Iron dagger";
const IRON_DAGGER_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 90),
    attack_power: Stat::new(StatType::AttackPower, 6),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronDagger {
    pub core: equippables::EquippableCore,
}

impl Describable for IronDagger {
    fn description(&self) -> &str {
        IRON_DAGGER_DESCRIPTION
    }
}

impl HasDisplayName for IronDagger {
    fn display_name(&self) -> &str {
        IRON_DAGGER_DISPLAY_NAME
    }
}

impl HasStats for IronDagger {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_DAGGER_DEFAULT_STATS])
    }
}
