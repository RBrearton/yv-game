use crate::prelude::*;

const WOOL_GLOVES_DESCRIPTION: &str = "Not very protective, but they will keep your hands warm.";
const WOOL_GLOVES_DISPLAY_NAME: &str = "Wool gloves";

const WOOL_GLOVES_DEFAULT_STATS: Stats = Stats {
    armour: Stat::new(StatType::Armour, 1),
    warmth: Stat::new(StatType::Warmth, 5),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoolGloves {
    pub core: equippables::EquippableCore,
}

impl Describable for WoolGloves {
    fn description(&self) -> &str {
        WOOL_GLOVES_DESCRIPTION
    }
}

impl HasDisplayName for WoolGloves {
    fn display_name(&self) -> &str {
        WOOL_GLOVES_DISPLAY_NAME
    }
}

impl HasStats for WoolGloves {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, WOOL_GLOVES_DEFAULT_STATS])
    }
}
