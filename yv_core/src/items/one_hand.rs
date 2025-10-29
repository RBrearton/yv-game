use crate::prelude::*;

/// # One hand
/// An enum containing all one hand slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OneHand {
    Dagger(Dagger),
    Hatchet(Hatchet),
    Pickaxe(Pickaxe),
}

impl Describable for OneHand {
    fn description(&self) -> &str {
        match self {
            OneHand::Dagger(dagger) => dagger.description(),
            OneHand::Hatchet(hatchet) => hatchet.description(),
            OneHand::Pickaxe(pickaxe) => pickaxe.description(),
        }
    }
}
impl HasDisplayName for OneHand {
    fn display_name(&self) -> &str {
        match self {
            OneHand::Dagger(dagger) => dagger.display_name(),
            OneHand::Hatchet(hatchet) => hatchet.display_name(),
            OneHand::Pickaxe(pickaxe) => pickaxe.display_name(),
        }
    }
}
