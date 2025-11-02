use crate::prelude::*;

const LINEN_TROUSERS_DESCRIPTION: &str = "Some simple linen trousers.";
const LINEN_TROUSERS_DISPLAY_NAME: &str = "Linen trousers";
const LINEN_TROUSERS_DEFAULT_STATS: Stats = Stats {
    warmth: Stat::new(StatType::Warmth, 2),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinenTrousers {
    pub core: equippables::EquippableCore,
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
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, LINEN_TROUSERS_DEFAULT_STATS])
    }
}
