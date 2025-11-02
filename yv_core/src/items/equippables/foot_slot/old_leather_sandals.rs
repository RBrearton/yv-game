use crate::prelude::*;

const OLD_LEATHER_SANDALS_DESCRIPTION: &str = "Some simple leather sandals. \
They're well used, and they don't look very protective, but they're better than nothing.";
const OLD_LEATHER_SANDALS_DISPLAY_NAME: &str = "Old leather sandals";
const OLD_LEATHER_SANDALS_DEFAULT_STATS: Stats = Stats {
    armour: Stat::new(StatType::Armour, 1),
    speed: Stat::new(StatType::Speed, 4),
    warmth: Stat::new(StatType::Warmth, 1),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct OldLeatherSandals {
    pub augmentation: Option<Augmentation>,
    pub durability: Durability,
}

impl Describable for OldLeatherSandals {
    fn description(&self) -> &str {
        OLD_LEATHER_SANDALS_DESCRIPTION
    }
}

impl HasDisplayName for OldLeatherSandals {
    fn display_name(&self) -> &str {
        OLD_LEATHER_SANDALS_DISPLAY_NAME
    }
}

impl HasStats for OldLeatherSandals {
    fn stats(&self) -> Stats {
        let augmentation_stats = self.augmentation.map(|augmentation| augmentation.stats());

        Stats::add([
            augmentation_stats.unwrap_or_default(),
            OLD_LEATHER_SANDALS_DEFAULT_STATS,
        ])
    }
}

impl Durable for OldLeatherSandals {
    fn durability(&self) -> &Durability {
        &self.durability
    }

    fn durability_mut(&mut self) -> &mut Durability {
        &mut self.durability
    }
}
