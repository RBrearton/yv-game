use crate::prelude::*;

const IRON_SKULLCAP_DESCRIPTION: &str = "A simple, lightweight iron helmet with good visibility.";
const IRON_SKULLCAP_DISPLAY_NAME: &str = "Iron skullcap";

/// # Iron skullcap
/// The struct containing the data required to represent an iron skullcap.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronSkullcap {
    pub stats: Stats,
}

impl Default for IronSkullcap {
    fn default() -> Self {
        Self {
            stats: Stats {
                armour: Stat::new(StatType::Armour, 5),
                speed: Stat::new(StatType::Speed, -1),
                ..Stats::default()
            },
        }
    }
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
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
