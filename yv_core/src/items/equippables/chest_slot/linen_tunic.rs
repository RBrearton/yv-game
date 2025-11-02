use crate::prelude::*;

const LINEN_TUNIC_DESCRIPTION: &str = "A simple linen tunic.";
const LINEN_TUNIC_DISPLAY_NAME: &str = "Linen tunic";
const LINEN_TUNIC_DEFAULT_STATS: Stats = Stats {
    armour: Stat::new(StatType::Armour, 1),
    warmth: Stat::new(StatType::Warmth, 3),
    ..Stats::empty()
};

/// # Linen tunic
/// The struct containing the data required to represent a linen tunic.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinenTunic {
    pub core: equippables::EquippableCore,
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
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, LINEN_TUNIC_DEFAULT_STATS])
    }
}
