use crate::prelude::*;

const IRON_ARROW_DESCRIPTION: &str = "A simple wooden arrow, with an iron head.";
const IRON_ARROW_DISPLAY_NAME: &str = "Iron arrow";
const IRON_ARROW_DEFAULT_STATS: Stats = Stats {
    ranged_damage: Stat::new(StatType::RangedDamage, 5),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronArrow {
    pub durability: Durability,
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
        IRON_ARROW_DEFAULT_STATS
    }
}

impl Durable for IronArrow {
    fn durability(&self) -> &Durability {
        &self.durability
    }

    fn durability_mut(&mut self) -> &mut Durability {
        &mut self.durability
    }
}
