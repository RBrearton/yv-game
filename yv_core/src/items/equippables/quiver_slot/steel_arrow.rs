use crate::prelude::*;

const STEEL_ARROW_DESCRIPTION: &str = "A simple wooden arrow, with a steel head.";
const STEEL_ARROW_DISPLAY_NAME: &str = "Steel arrow";
const STEEL_ARROW_DEFAULT_STATS: Stats = Stats {
    ranged_damage: Stat::new(StatType::RangedDamage, 8),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SteelArrow {
    pub durability: Durability,
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
        STEEL_ARROW_DEFAULT_STATS
    }
}

impl Durable for SteelArrow {
    fn durability(&self) -> &Durability {
        &self.durability
    }

    fn durability_mut(&mut self) -> &mut Durability {
        &mut self.durability
    }
}
