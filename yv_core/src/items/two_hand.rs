use crate::prelude::*;

/// # Two hand
/// An enum containing all two hand slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TwoHand {
    Greatsword(Greatsword),
    Battleaxe(Battleaxe),
}

impl Describable for TwoHand {
    fn description(&self) -> &str {
        match self {
            TwoHand::Greatsword(greatsword) => greatsword.description(),
            TwoHand::Battleaxe(battleaxe) => battleaxe.description(),
        }
    }
}
impl HasDisplayName for TwoHand {
    fn display_name(&self) -> &str {
        match self {
            TwoHand::Greatsword(greatsword) => greatsword.display_name(),
            TwoHand::Battleaxe(battleaxe) => battleaxe.display_name(),
        }
    }
}

impl HasStats for TwoHand {
    fn stats(&self) -> &Stats {
        match self {
            TwoHand::Greatsword(greatsword) => greatsword.stats(),
            TwoHand::Battleaxe(battleaxe) => battleaxe.stats(),
        }
    }
}
