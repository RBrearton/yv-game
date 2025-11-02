use crate::prelude::*;

const RAGGED_CLOAK_DESCRIPTION: &str = "A simple cloak that has seen better days.";
const RAGGED_CLOAK_DISPLAY_NAME: &str = "Ragged cloak";

const RAGGED_CLOAK_STATS: Stats = Stats {
    armour: Stat::new(StatType::Armour, 1),
    warmth: Stat::new(StatType::Warmth, 4),
    speed: Stat::new(StatType::Speed, -3),
    ..Stats::empty()
};

#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RaggedCloak {
    pub core: equippables::EquippableCore,
}

impl Describable for RaggedCloak {
    fn description(&self) -> &str {
        RAGGED_CLOAK_DESCRIPTION
    }
}

impl HasDisplayName for RaggedCloak {
    fn display_name(&self) -> &str {
        RAGGED_CLOAK_DISPLAY_NAME
    }
}

impl HasStats for RaggedCloak {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, RAGGED_CLOAK_STATS])
    }
}
