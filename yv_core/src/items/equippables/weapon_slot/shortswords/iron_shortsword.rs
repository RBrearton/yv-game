use crate::prelude::*;

const IRON_SHORTSWORD_DESCRIPTION: &str = "A simple iron shortsword.";
const IRON_SHORTSWORD_DISPLAY_NAME: &str = "Iron shortsword";
const IRON_SHORTSWORD_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 75),
    attack_power: Stat::new(StatType::AttackPower, 13),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronShortsword {
    pub core: equippables::EquippableCore,
}

impl Describable for IronShortsword {
    fn description(&self) -> &str {
        IRON_SHORTSWORD_DESCRIPTION
    }
}

impl HasDisplayName for IronShortsword {
    fn display_name(&self) -> &str {
        IRON_SHORTSWORD_DISPLAY_NAME
    }
}

impl HasStats for IronShortsword {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_SHORTSWORD_DEFAULT_STATS])
    }
}

impl Durable for IronShortsword {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
