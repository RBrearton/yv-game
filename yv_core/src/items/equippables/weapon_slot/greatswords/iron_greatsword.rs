use crate::prelude::*;

const IRON_GREATSWORD_DESCRIPTION: &str = "A large iron greatsword.";
const IRON_GREATSWORD_DISPLAY_NAME: &str = "Iron greatsword";
const IRON_GREATSWORD_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 30),
    attack_power: Stat::new(StatType::AttackPower, 35),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronGreatsword {
    pub core: equippables::EquippableCore,
}

impl Describable for IronGreatsword {
    fn description(&self) -> &str {
        IRON_GREATSWORD_DESCRIPTION
    }
}

impl HasDisplayName for IronGreatsword {
    fn display_name(&self) -> &str {
        IRON_GREATSWORD_DISPLAY_NAME
    }
}

impl HasStats for IronGreatsword {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_GREATSWORD_DEFAULT_STATS])
    }
}

impl Durable for IronGreatsword {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
