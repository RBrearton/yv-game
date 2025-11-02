use crate::prelude::*;

const STONE_ARROW_DESCRIPTION: &str = "A simple wooden arrow, with a stone head.";
const STONE_ARROW_DISPLAY_NAME: &str = "Stone arrow";
const STONE_ARROW_DEFAULT_STATS: Stats = Stats {
    ranged_damage: Stat::new(StatType::RangedDamage, 2),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneArrow {
    pub durability: Durability,
}

impl Describable for StoneArrow {
    fn description(&self) -> &str {
        STONE_ARROW_DESCRIPTION
    }
}

impl HasDisplayName for StoneArrow {
    fn display_name(&self) -> &str {
        STONE_ARROW_DISPLAY_NAME
    }
}

impl HasStats for StoneArrow {
    fn stats(&self) -> Stats {
        STONE_ARROW_DEFAULT_STATS
    }
}

impl Durable for StoneArrow {
    fn durability(&self) -> &Durability {
        &self.durability
    }

    fn durability_mut(&mut self) -> &mut Durability {
        &mut self.durability
    }
}
