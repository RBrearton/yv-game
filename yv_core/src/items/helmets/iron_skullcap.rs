use crate::prelude::*;

const IRON_SKULLCAP_DESCRIPTION: &str = "An iron skullcap.";
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
                armour: Stat::new(StatType::Armour, 3.0),
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
