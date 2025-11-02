use crate::prelude::*;

const STONE_IDOL_DESCRIPTION: &str = "A small stone idol.";
const STONE_IDOL_DISPLAY_NAME: &str = "Stone idol";
const STONE_IDOL_DEFAULT_STATS: Stats = Stats {
    speed: Stat::new(StatType::Speed, -1),
    ..Stats::empty()
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoneIdol {
    pub core: equippables::EquippableCore,
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
    fn stats(&self) -> Stats {
        let core_stats = self.core.stats();
        Stats::add([core_stats, STONE_IDOL_DEFAULT_STATS])
    }
}
