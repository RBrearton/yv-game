use crate::prelude::*;

/// # Footwear
/// An enum containing all feet slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Footwear {
    OldLeatherSandals(equippables::foot_slot::OldLeatherSandals),
}

impl Describable for Footwear {
    fn description(&self) -> &str {
        match self {
            Footwear::OldLeatherSandals(old_leather_sandals) => old_leather_sandals.description(),
        }
    }
}

impl HasDisplayName for Footwear {
    fn display_name(&self) -> &str {
        match self {
            Footwear::OldLeatherSandals(old_leather_sandals) => old_leather_sandals.display_name(),
        }
    }
}

impl HasStats for Footwear {
    fn stats(&self) -> Stats {
        match self {
            Footwear::OldLeatherSandals(old_leather_sandals) => old_leather_sandals.stats(),
        }
    }
}

impl Durable for Footwear {
    fn durability(&self) -> &Durability {
        match self {
            Footwear::OldLeatherSandals(old_leather_sandals) => old_leather_sandals.durability(),
        }
    }

    fn durability_mut(&mut self) -> &mut Durability {
        match self {
            Footwear::OldLeatherSandals(old_leather_sandals) => {
                old_leather_sandals.durability_mut()
            }
        }
    }
}
