use crate::prelude::*;

const WOOL_GLOVES_DESCRIPTION: &str = "Not very protective, but they will keep your hands warm.";
const WOOL_GLOVES_DISPLAY_NAME: &str = "Wool gloves";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WoolGloves {
    pub stats: Stats,
}

impl Default for WoolGloves {
    fn default() -> Self {
        Self {
            stats: Stats {
                armour: Stat::new(StatType::Armour, 1),
                warmth: Stat::new(StatType::Warmth, 2),
                ..Stats::default()
            },
        }
    }
}

impl Describable for WoolGloves {
    fn description(&self) -> &str {
        WOOL_GLOVES_DESCRIPTION
    }
}

impl HasDisplayName for WoolGloves {
    fn display_name(&self) -> &str {
        WOOL_GLOVES_DISPLAY_NAME
    }
}

impl HasStats for WoolGloves {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
