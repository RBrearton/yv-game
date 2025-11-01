use crate::prelude::*;

const WOODEN_BUCKLER_DESCRIPTION: &str = "A simple wooden buckler.";
const WOODEN_BUCKLER_DISPLAY_NAME: &str = "Wooden buckler";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoodenBuckler {
    pub stats: Stats,
}

impl Default for WoodenBuckler {
    fn default() -> Self {
        Self {
            stats: Stats {
                armour: Stat::new(StatType::Armour, 6),
                block_points: Stat::new(StatType::BlockPoints, 30),
                ..Stats::default()
            },
        }
    }
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
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
