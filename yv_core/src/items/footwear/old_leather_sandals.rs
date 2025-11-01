use crate::prelude::*;

const OLD_LEATHER_SANDALS_DESCRIPTION: &str = "Some simple leather sandals. \
They're well used, and they don't look very protective, but they're better than nothing.";
const OLD_LEATHER_SANDALS_DISPLAY_NAME: &str = "Old leather sandals";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct OldLeatherSandals {
    pub stats: Stats,
}

impl Default for OldLeatherSandals {
    fn default() -> Self {
        Self {
            stats: Stats {
                armour: Stat::new(StatType::Armour, 1),
                speed: Stat::new(StatType::Speed, 4),
                warmth: Stat::new(StatType::Warmth, 1),
                ..Stats::default()
            },
        }
    }
}

impl Describable for OldLeatherSandals {
    fn description(&self) -> &str {
        OLD_LEATHER_SANDALS_DESCRIPTION
    }
}

impl HasDisplayName for OldLeatherSandals {
    fn display_name(&self) -> &str {
        OLD_LEATHER_SANDALS_DISPLAY_NAME
    }
}

impl HasStats for OldLeatherSandals {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
