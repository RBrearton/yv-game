use crate::prelude::*;

const LEATHER_TUNIC_DESCRIPTION: &str = "A simple leather tunic.";
const LEATHER_TUNIC_DISPLAY_NAME: &str = "Leather tunic";

/// # Leather tunic
/// The struct containing the data required to represent a leather tunic.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeatherTunic {
    pub stats: Stats,
}

impl Default for LeatherTunic {
    fn default() -> Self {
        Self {
            stats: Stats {
                armour: Stat::new(StatType::Armour, 2.0),
                ..Stats::default()
            },
        }
    }
}

impl Describable for LeatherTunic {
    fn description(&self) -> &str {
        LEATHER_TUNIC_DESCRIPTION
    }
}
impl HasDisplayName for LeatherTunic {
    fn display_name(&self) -> &str {
        LEATHER_TUNIC_DISPLAY_NAME
    }
}
impl HasStats for LeatherTunic {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
