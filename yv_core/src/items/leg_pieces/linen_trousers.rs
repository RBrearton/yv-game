use crate::prelude::*;

const LINEN_TROUSERS_DESCRIPTION: &str = "Some simple linen trousers.";
const LINEN_TROUSERS_DISPLAY_NAME: &str = "Linen trousers";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinenTrousers {
    pub stats: Stats,
}

impl Default for LinenTrousers {
    fn default() -> Self {
        Self {
            stats: Stats {
                warmth: Stat::new(StatType::Warmth, 2),
                ..Stats::default()
            },
        }
    }
}

impl Describable for LinenTrousers {
    fn description(&self) -> &str {
        LINEN_TROUSERS_DESCRIPTION
    }
}

impl HasDisplayName for LinenTrousers {
    fn display_name(&self) -> &str {
        LINEN_TROUSERS_DISPLAY_NAME
    }
}

impl HasStats for LinenTrousers {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
