use crate::prelude::*;

const IRON_BATTLEAXE_DESCRIPTION: &str = "A simple iron battle axe.";
const IRON_BATTLEAXE_DISPLAY_NAME: &str = "Iron battleaxe";
const IRON_BATTLEAXE_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 38),
    attack_power: Stat::new(StatType::AttackPower, 19),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronBattleaxe {
    pub core: equippables::EquippableCore,
}

impl Describable for IronBattleaxe {
    fn description(&self) -> &str {
        IRON_BATTLEAXE_DESCRIPTION
    }
}

impl HasDisplayName for IronBattleaxe {
    fn display_name(&self) -> &str {
        IRON_BATTLEAXE_DISPLAY_NAME
    }
}

impl HasStats for IronBattleaxe {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_BATTLEAXE_DEFAULT_STATS])
    }
}

impl Durable for IronBattleaxe {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
