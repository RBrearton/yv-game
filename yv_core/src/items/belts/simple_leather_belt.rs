use crate::prelude::*;

const SIMPLE_LEATHER_BELT_DESCRIPTION: &str = "A simple leather belt.";
const SIMPLE_LEATHER_BELT_DISPLAY_NAME: &str = "Simple leather belt";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleLeatherBelt {
    pub stats: Stats,
}

impl Default for SimpleLeatherBelt {
    fn default() -> Self {
        Self {
            stats: Stats {
                speed: Stat::new(StatType::Speed, 1),
                ..Stats::default()
            },
        }
    }
}

impl Describable for SimpleLeatherBelt {
    fn description(&self) -> &str {
        SIMPLE_LEATHER_BELT_DESCRIPTION
    }
}

impl HasDisplayName for SimpleLeatherBelt {
    fn display_name(&self) -> &str {
        SIMPLE_LEATHER_BELT_DISPLAY_NAME
    }
}

impl HasStats for SimpleLeatherBelt {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
