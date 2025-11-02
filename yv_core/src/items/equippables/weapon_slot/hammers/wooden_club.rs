use crate::prelude::*;

const WOODEN_CLUB_DESCRIPTION: &str = "A heavy wooden club.";
const WOODEN_CLUB_DISPLAY_NAME: &str = "Wooden club";
const WOODEN_CLUB_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 54),
    attack_power: Stat::new(StatType::AttackPower, 11),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoodenClub {
    pub core: equippables::EquippableCore,
}

impl Describable for WoodenClub {
    fn description(&self) -> &str {
        WOODEN_CLUB_DESCRIPTION
    }
}

impl HasDisplayName for WoodenClub {
    fn display_name(&self) -> &str {
        WOODEN_CLUB_DISPLAY_NAME
    }
}

impl HasStats for WoodenClub {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, WOODEN_CLUB_DEFAULT_STATS])
    }
}

impl Durable for WoodenClub {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
