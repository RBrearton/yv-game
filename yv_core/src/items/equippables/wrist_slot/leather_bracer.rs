use crate::prelude::*;

const LEATHER_BRACER_DESCRIPTION: &str = "A simple leather bracer.";
const LEATHER_BRACER_DISPLAY_NAME: &str = "Leather bracer";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeatherBracer {
    pub stats: Stats,
}

impl Default for LeatherBracer {
    fn default() -> Self {
        Self {
            stats: Stats {
                ranged_accuracy: Stat::new(StatType::RangedAccuracy, 4),
                ..Stats::default()
            },
        }
    }
}

impl Describable for LeatherBracer {
    fn description(&self) -> &str {
        LEATHER_BRACER_DESCRIPTION
    }
}

impl HasDisplayName for LeatherBracer {
    fn display_name(&self) -> &str {
        LEATHER_BRACER_DISPLAY_NAME
    }
}

impl HasStats for LeatherBracer {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
