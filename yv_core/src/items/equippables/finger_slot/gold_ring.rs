use crate::prelude::*;

const GOLD_RING_DESCRIPTION: &str = "A plain gold band.";
const GOLD_RING_DISPLAY_NAME: &str = "Gold ring";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GoldRing {
    pub stats: Stats,
}

impl Describable for GoldRing {
    fn description(&self) -> &str {
        GOLD_RING_DESCRIPTION
    }
}

impl HasDisplayName for GoldRing {
    fn display_name(&self) -> &str {
        GOLD_RING_DISPLAY_NAME
    }
}

impl HasStats for GoldRing {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
