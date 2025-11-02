use crate::prelude::*;

const WOODEN_BUCKLER_DESCRIPTION: &str = "A simple wooden buckler.";
const WOODEN_BUCKLER_DISPLAY_NAME: &str = "Wooden buckler";
const WOODEN_BUCKLER_DEFAULT_STATS: Stats = Stats {
    armour: Stat::new(StatType::Armour, 6),
    block_points: Stat::new(StatType::BlockPoints, 30),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoodenBuckler {
    pub core: equippables::EquippableCore,
}

impl Describable for WoodenBuckler {
    fn description(&self) -> &str {
        WOODEN_BUCKLER_DESCRIPTION
    }
}

impl HasDisplayName for WoodenBuckler {
    fn display_name(&self) -> &str {
        WOODEN_BUCKLER_DISPLAY_NAME
    }
}

impl HasStats for WoodenBuckler {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, WOODEN_BUCKLER_DEFAULT_STATS])
    }
}

impl Durable for WoodenBuckler {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
