use crate::prelude::*;

const STONE_IDOL_DESCRIPTION: &str = "A small stone idol.";
const STONE_IDOL_DISPLAY_NAME: &str = "Stone idol";

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneIdol {
    pub stats: Stats,
}

impl Default for StoneIdol {
    fn default() -> Self {
        Self {
            stats: Stats {
                speed: Stat::new(StatType::Speed, -1),
                ..Default::default()
            },
        }
    }
}

impl Describable for StoneIdol {
    fn description(&self) -> &str {
        STONE_IDOL_DESCRIPTION
    }
}

impl HasDisplayName for StoneIdol {
    fn display_name(&self) -> &str {
        STONE_IDOL_DISPLAY_NAME
    }
}

impl HasStats for StoneIdol {
    fn stats(&self) -> &Stats {
        &self.stats
    }
}
