use crate::prelude::*;

const SIMPLE_LEATHER_BELT_DESCRIPTION: &str = "A simple leather belt.";
const SIMPLE_LEATHER_BELT_DISPLAY_NAME: &str = "Simple leather belt";
const SIMPLE_LEATHER_BELT_STATS: Stats = Stats {
    speed: Stat::new(StatType::Speed, 1),
    ..Stats::empty()
};

#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleLeatherBelt {
    pub core: equippables::EquippableCore,
}

impl Describable for SimpleLeatherBelt {
    fn description(&self) -> &str {
        SIMPLE_LEATHER_BELT_DESCRIPTION
    }
}

impl HasDisplayName for SimpleLeatherBelt {
    fn display_name(&self) -> &str {
        SIMPLE_LEATHER_BELT_DISPLAY_NAME
    }
}

impl HasStats for SimpleLeatherBelt {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, SIMPLE_LEATHER_BELT_STATS])
    }
}

impl Durable for SimpleLeatherBelt {
    fn durability(&self) -> &Durability {
        self.core.durability()
    }

    fn durability_mut(&mut self) -> &mut Durability {
        self.core.durability_mut()
    }
}
