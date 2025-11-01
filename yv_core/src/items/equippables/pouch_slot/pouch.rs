use crate::prelude::*;

/// # Pouch
/// An enum containing all pouch slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Pouch {
    StoneIdol(equippables::pouch_slot::StoneIdol),
}

impl Describable for Pouch {
    fn description(&self) -> &str {
        match self {
            Pouch::StoneIdol(stone_idol) => stone_idol.description(),
        }
    }
}

impl HasDisplayName for Pouch {
    fn display_name(&self) -> &str {
        match self {
            Pouch::StoneIdol(stone_idol) => stone_idol.display_name(),
        }
    }
}

impl HasStats for Pouch {
    fn stats(&self) -> &Stats {
        match self {
            Pouch::StoneIdol(stone_idol) => stone_idol.stats(),
        }
    }
}
