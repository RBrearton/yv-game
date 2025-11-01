use crate::prelude::*;

const STEEL_ARROW_DESCRIPTION: &str = "A simple wooden arrow, with a steel head.";
const STEEL_ARROW_DISPLAY_NAME: &str = "Steel arrow";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SteelArrow {
    pub stats: Stats,
}

impl Default for SteelArrow {
    fn default() -> Self {
        Self {
            stats: Stats {
                ranged_damage: Stat::new(StatType::RangedDamage, 8),
                ..Stats::default()
            },
        }
    }
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
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
