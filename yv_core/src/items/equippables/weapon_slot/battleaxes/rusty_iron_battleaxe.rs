use crate::prelude::*;

const RUSTY_IRON_BATTLEAXE_DESCRIPTION: &str = "An iron battle axe that has seen better days.";
const RUSTY_IRON_BATTLEAXE_DISPLAY_NAME: &str = "Rusty iron battleaxe";
const RUSTY_IRON_BATTLEAXE_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 38),
    attack_power: Stat::new(StatType::AttackPower, 14),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RustyIronBattleaxe {
    pub core: equippables::EquippableCore,
}

impl Describable for RustyIronBattleaxe {
    fn description(&self) -> &str {
        RUSTY_IRON_BATTLEAXE_DESCRIPTION
    }
}

impl HasDisplayName for RustyIronBattleaxe {
    fn display_name(&self) -> &str {
        RUSTY_IRON_BATTLEAXE_DISPLAY_NAME
    }
}

impl HasStats for RustyIronBattleaxe {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, RUSTY_IRON_BATTLEAXE_DEFAULT_STATS])
    }
}

impl Durable for RustyIronBattleaxe {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
