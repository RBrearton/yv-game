use crate::prelude::*;

const WOODEN_DAGGER_DESCRIPTION: &str = "A simple wooden dagger.";
const WOODEN_DAGGER_DISPLAY_NAME: &str = "Wooden dagger";
const WOODEN_DAGGER_DEFAULT_STATS: Stats = Stats {
    weapon_speed: Stat::new(StatType::WeaponSpeed, 90),
    attack_power: Stat::new(StatType::AttackPower, 1),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoodenDagger {
    pub augmentation: Option<Augmentation>,
    pub durability: Durability,
}

impl Describable for WoodenDagger {
    fn description(&self) -> &str {
        WOODEN_DAGGER_DESCRIPTION
    }
}

impl HasDisplayName for WoodenDagger {
    fn display_name(&self) -> &str {
        WOODEN_DAGGER_DISPLAY_NAME
    }
}

impl HasStats for WoodenDagger {
    fn stats(&self) -> Stats {
        let augmentation_stats = self.augmentation.map(|augmentation| augmentation.stats());

        Stats::add([
            augmentation_stats.unwrap_or_default(),
            WOODEN_DAGGER_DEFAULT_STATS,
        ])
    }
}

impl Durable for WoodenDagger {
    fn durability(&self) -> &Durability {
        &self.durability
    }

    fn durability_mut(&mut self) -> &mut Durability {
        &mut self.durability
    }
}
