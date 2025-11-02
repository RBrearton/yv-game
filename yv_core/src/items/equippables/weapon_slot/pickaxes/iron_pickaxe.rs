use crate::prelude::*;

const IRON_PICKAXE_DESCRIPTION: &str = "A simple iron pickaxe.";
const IRON_PICKAXE_DISPLAY_NAME: &str = "Iron pickaxe";
const IRON_PICKAXE_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 45),
    attack_power: Stat::new(StatType::AttackPower, 6),
    mining_power: Stat::new(StatType::MiningPower, 4),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronPickaxe {
    pub core: equippables::EquippableCore,
}

impl Describable for IronPickaxe {
    fn description(&self) -> &str {
        IRON_PICKAXE_DESCRIPTION
    }
}

impl HasDisplayName for IronPickaxe {
    fn display_name(&self) -> &str {
        IRON_PICKAXE_DISPLAY_NAME
    }
}

impl HasStats for IronPickaxe {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_PICKAXE_DEFAULT_STATS])
    }
}

impl Durable for IronPickaxe {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
