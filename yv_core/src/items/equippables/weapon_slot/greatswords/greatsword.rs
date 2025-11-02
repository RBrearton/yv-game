use crate::prelude::*;

/// # Greatsword
/// An enum containing all greatsword slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Greatsword {
    IronGreatsword(equippables::weapon_slot::greatswords::IronGreatsword),
}

impl Describable for Greatsword {
    fn description(&self) -> &str {
        match self {
            Greatsword::IronGreatsword(iron_greatsword) => iron_greatsword.description(),
        }
    }
}

impl HasDisplayName for Greatsword {
    fn display_name(&self) -> &str {
        match self {
            Greatsword::IronGreatsword(iron_greatsword) => iron_greatsword.display_name(),
        }
    }
}

impl HasStats for Greatsword {
    fn stats(&self) -> Stats {
        match self {
            Greatsword::IronGreatsword(iron_greatsword) => iron_greatsword.stats(),
        }
    }
}

impl Durable for Greatsword {
    fn durability(&self) -> &Durability {
        match self {
            Greatsword::IronGreatsword(iron_greatsword) => iron_greatsword.durability(),
        }
    }

    fn durability_mut(&mut self) -> &mut Durability {
        match self {
            Greatsword::IronGreatsword(iron_greatsword) => iron_greatsword.durability_mut(),
        }
    }
}
