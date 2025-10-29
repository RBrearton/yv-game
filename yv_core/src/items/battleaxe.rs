use crate::prelude::*;

const IRON_BATTLEAXE_DESCRIPTION: &str = "A sturdy looking battleaxe made of iron.";
const IRON_BATTLEAXE_DISPLAY_NAME: &str = "Iron battleaxe";

/// # Battleaxe
/// An enum containing all battleaxe slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Battleaxe {
    IronBattleaxe,
}

impl Describable for Battleaxe {
    fn description(&self) -> &str {
        match self {
            Battleaxe::IronBattleaxe => IRON_BATTLEAXE_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Battleaxe {
    fn display_name(&self) -> &str {
        match self {
            Battleaxe::IronBattleaxe => IRON_BATTLEAXE_DISPLAY_NAME,
        }
    }
}
