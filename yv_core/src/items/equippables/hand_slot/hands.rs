use crate::prelude::*;

/// # Hands
/// An enum containing all hands slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Hands {
    WoolGloves(equippables::hand_slot::WoolGloves),
}

impl Describable for Hands {
    fn description(&self) -> &str {
        match self {
            Hands::WoolGloves(wool_gloves) => wool_gloves.description(),
        }
    }
}

impl HasDisplayName for Hands {
    fn display_name(&self) -> &str {
        match self {
            Hands::WoolGloves(wool_gloves) => wool_gloves.display_name(),
        }
    }
}

impl HasStats for Hands {
    fn stats(&self) -> Stats {
        match self {
            Hands::WoolGloves(wool_gloves) => wool_gloves.stats(),
        }
    }
}

impl Durable for Hands {
    fn durability(&self) -> &Durability {
        match self {
            Hands::WoolGloves(wool_gloves) => wool_gloves.durability(),
        }
    }

    fn durability_mut(&mut self) -> &mut Durability {
        match self {
            Hands::WoolGloves(wool_gloves) => wool_gloves.durability_mut(),
        }
    }
}
