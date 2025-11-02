use crate::prelude::*;

/// # Shield
/// An enum containing all shield slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shield {
    WoodenBuckler(equippables::weapon_slot::shields::WoodenBuckler),
}

impl Describable for Shield {
    fn description(&self) -> &str {
        match self {
            Shield::WoodenBuckler(wooden_buckler) => wooden_buckler.description(),
        }
    }
}

impl HasDisplayName for Shield {
    fn display_name(&self) -> &str {
        match self {
            Shield::WoodenBuckler(wooden_buckler) => wooden_buckler.display_name(),
        }
    }
}

impl HasStats for Shield {
    fn stats(&self) -> Stats {
        match self {
            Shield::WoodenBuckler(wooden_buckler) => wooden_buckler.stats(),
        }
    }
}

impl Durable for Shield {
    fn durability(&self) -> &Durability {
        match self {
            Shield::WoodenBuckler(wooden_buckler) => wooden_buckler.durability(),
        }
    }

    fn durability_mut(&mut self) -> &mut Durability {
        match self {
            Shield::WoodenBuckler(wooden_buckler) => wooden_buckler.durability_mut(),
        }
    }
}
