use crate::prelude::*;

/// # One hand
/// An enum containing all one hand slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OneHand {
    Dagger(Dagger),
    Hammer(Hammer),
    Hatchet(Hatchet),
    Pickaxe(Pickaxe),
    Shortsword(Shortsword),
}

impl Describable for OneHand {
    fn description(&self) -> &str {
        match self {
            OneHand::Dagger(dagger) => dagger.description(),
            OneHand::Hammer(hammer) => hammer.description(),
            OneHand::Hatchet(hatchet) => hatchet.description(),
            OneHand::Pickaxe(pickaxe) => pickaxe.description(),
            OneHand::Shortsword(shortsword) => shortsword.description(),
        }
    }
}
impl HasDisplayName for OneHand {
    fn display_name(&self) -> &str {
        match self {
            OneHand::Dagger(dagger) => dagger.display_name(),
            OneHand::Hammer(hammer) => hammer.display_name(),
            OneHand::Hatchet(hatchet) => hatchet.display_name(),
            OneHand::Pickaxe(pickaxe) => pickaxe.display_name(),
            OneHand::Shortsword(shortsword) => shortsword.display_name(),
        }
    }
}

impl HasStats for OneHand {
    fn stats(&self) -> &Stats {
        match self {
            OneHand::Dagger(dagger) => dagger.stats(),
            OneHand::Hammer(hammer) => hammer.stats(),
            OneHand::Hatchet(hatchet) => hatchet.stats(),
            OneHand::Pickaxe(pickaxe) => pickaxe.stats(),
            OneHand::Shortsword(shortsword) => shortsword.stats(),
        }
    }
}
