use crate::prelude::*;

const GOLD_RING_DESCRIPTION: &str = "A plain gold band.";
const GOLD_RING_DISPLAY_NAME: &str = "Gold ring";
const GOLD_RING_DEFAULT_STATS: Stats = Stats {
    // No stats on an empty gold ring.
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GoldRing {
    pub enchantment_1: Option<Enchantment>,
    pub enchantment_2: Option<Enchantment>,
}

impl Describable for GoldRing {
    fn description(&self) -> &str {
        GOLD_RING_DESCRIPTION
    }
}

impl HasDisplayName for GoldRing {
    fn display_name(&self) -> &str {
        GOLD_RING_DISPLAY_NAME
    }
}

impl HasStats for GoldRing {
    fn stats(&self) -> Stats {
        let enchantment_1_stats = self.enchantment_1.map(|enchantment| enchantment.stats());
        let enchantment_2_stats = self.enchantment_2.map(|enchantment| enchantment.stats());
        let stats = Stats::add([
            enchantment_1_stats.unwrap_or_default(),
            enchantment_2_stats.unwrap_or_default(),
        ]);
        stats
    }
}
