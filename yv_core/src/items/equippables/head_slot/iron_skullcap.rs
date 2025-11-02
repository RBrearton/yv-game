use crate::prelude::*;

const IRON_SKULLCAP_DESCRIPTION: &str = "A simple, lightweight iron helmet with good visibility.";
const IRON_SKULLCAP_DISPLAY_NAME: &str = "Iron skullcap";
const IRON_SKULLCAP_DEFAULT_STATS: Stats = Stats {
    armour: Stat::new(StatType::Armour, 5),
    speed: Stat::new(StatType::Speed, -1),
    precision: Stat::new(StatType::Precision, -3),
    ..Stats::empty()
};

/// # Iron skullcap
/// The struct containing the data required to represent an iron skullcap.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronSkullcap {
    pub core: equippables::EquippableCore,
}

impl Describable for IronSkullcap {
    fn description(&self) -> &str {
        IRON_SKULLCAP_DESCRIPTION
    }
}
impl HasDisplayName for IronSkullcap {
    fn display_name(&self) -> &str {
        IRON_SKULLCAP_DISPLAY_NAME
    }
}
impl HasStats for IronSkullcap {
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, IRON_SKULLCAP_DEFAULT_STATS])
    }
}

impl Durable for IronSkullcap {
    fn current_durability(&self) -> u8 {
        self.core.current_durability()
    }

    fn max_durability(&self) -> u8 {
        self.core.max_durability()
    }

    fn decrease_durability(&mut self) {
        self.core.decrease_durability()
    }

    fn restore_durability(&mut self) {
        self.core.restore_durability()
    }
}
