use crate::prelude::*;

const LEATHER_BRACER_DESCRIPTION: &str = "A simple leather bracer.";
const LEATHER_BRACER_DISPLAY_NAME: &str = "Leather bracer";
const LEATHER_BRACER_DEFAULT_STATS: Stats = Stats {
    ranged_accuracy: Stat::new(StatType::RangedAccuracy, 4),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeatherBracer {
    pub core: equippables::EquippableCore,
}

impl Describable for LeatherBracer {
    fn description(&self) -> &str {
        LEATHER_BRACER_DESCRIPTION
    }
}

impl HasDisplayName for LeatherBracer {
    fn display_name(&self) -> &str {
        LEATHER_BRACER_DISPLAY_NAME
    }
}

impl HasStats for LeatherBracer {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, LEATHER_BRACER_DEFAULT_STATS])
    }
}

impl Durable for LeatherBracer {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
