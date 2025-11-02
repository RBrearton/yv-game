use crate::prelude::*;

const STEEL_ARROW_DESCRIPTION: &str = "A simple wooden arrow, with a steel head.";
const STEEL_ARROW_DISPLAY_NAME: &str = "Steel arrow";
const STEEL_ARROW_DEFAULT_STATS: Stats = Stats {
    ranged_damage: Stat::new(StatType::RangedDamage, 8),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SteelArrow {
    pub core: equippables::EquippableCore,
}

impl Describable for SteelArrow {
    fn description(&self) -> &str {
        STEEL_ARROW_DESCRIPTION
    }
}

impl HasDisplayName for SteelArrow {
    fn display_name(&self) -> &str {
        STEEL_ARROW_DISPLAY_NAME
    }
}

impl HasStats for SteelArrow {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, STEEL_ARROW_DEFAULT_STATS])
    }
}
