use crate::prelude::*;

const IRON_ARROW_DESCRIPTION: &str = "A simple wooden arrow, with an iron head.";
const IRON_ARROW_DISPLAY_NAME: &str = "Iron arrow";
const IRON_ARROW_DEFAULT_STATS: Stats = Stats {
    ranged_damage: Stat::new(StatType::RangedDamage, 5),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronArrow {
    pub core: equippables::EquippableCore,
}

impl Describable for IronArrow {
    fn description(&self) -> &str {
        IRON_ARROW_DESCRIPTION
    }
}

impl HasDisplayName for IronArrow {
    fn display_name(&self) -> &str {
        IRON_ARROW_DISPLAY_NAME
    }
}

impl HasStats for IronArrow {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_ARROW_DEFAULT_STATS])
    }
}
