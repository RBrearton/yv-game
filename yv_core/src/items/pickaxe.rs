use crate::prelude::*;

const STONE_PICKAXE_DESCRIPTION: &str = "A stone pickaxe.";
const STONE_PICKAXE_DISPLAY_NAME: &str = "Stone pickaxe";

/// # Pickaxe
/// An enum containing all pickaxe slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Pickaxe {
    StonePickaxe,
}

impl Describable for Pickaxe {
    fn description(&self) -> &str {
        match self {
            Pickaxe::StonePickaxe => STONE_PICKAXE_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Pickaxe {
    fn display_name(&self) -> &str {
        match self {
            Pickaxe::StonePickaxe => STONE_PICKAXE_DISPLAY_NAME,
        }
    }
}
