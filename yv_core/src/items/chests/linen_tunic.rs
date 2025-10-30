use crate::prelude::*;

const LINEN_TUNIC_DESCRIPTION: &str = "A simple linen tunic.";
const LINEN_TUNIC_DISPLAY_NAME: &str = "Linen tunic";

/// # Linen tunic
/// The struct containing the data required to represent a linen tunic.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinenTunic {
    pub stats: Stats,
}

impl Default for LinenTunic {
    fn default() -> Self {
        Self {
            stats: Stats {
                armour: Stat::new(StatType::Armour, 1.0),
                ..Stats::default()
            },
        }
    }
}

impl Describable for LinenTunic {
    fn description(&self) -> &str {
        LINEN_TUNIC_DESCRIPTION
    }
}
impl HasDisplayName for LinenTunic {
    fn display_name(&self) -> &str {
        LINEN_TUNIC_DISPLAY_NAME
    }
}
impl HasStats for LinenTunic {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
