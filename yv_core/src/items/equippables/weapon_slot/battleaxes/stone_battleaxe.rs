use crate::prelude::*;

const STONE_BATTLEAXE_DESCRIPTION: &str = "A crude stone battle axe.";
const STONE_BATTLEAXE_DISPLAY_NAME: &str = "Stone battleaxe";
const STONE_BATTLEAXE_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 38),
    attack_power: Stat::new(StatType::AttackPower, 12),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneBattleaxe {
    pub core: equippables::EquippableCore,
}

impl Describable for StoneBattleaxe {
    fn description(&self) -> &str {
        STONE_BATTLEAXE_DESCRIPTION
    }
}

impl HasDisplayName for StoneBattleaxe {
    fn display_name(&self) -> &str {
        STONE_BATTLEAXE_DISPLAY_NAME
    }
}

impl HasStats for StoneBattleaxe {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, STONE_BATTLEAXE_DEFAULT_STATS])
    }
}

impl Durable for StoneBattleaxe {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
