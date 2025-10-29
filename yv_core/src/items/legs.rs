use crate::prelude::*;

const LINEN_TROUSERS_DESCRIPTION: &str = "A simple linen tunic.";
const LINEN_TROUSERS_DISPLAY_NAME: &str = "Linen trousers";

/// # Legs
/// An enum containing all legs slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Legs {
    LinenTrousers,
}

impl Describable for Legs {
    fn description(&self) -> &str {
        match self {
            Legs::LinenTrousers => LINEN_TROUSERS_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Legs {
    fn display_name(&self) -> &str {
        match self {
            Legs::LinenTrousers => LINEN_TROUSERS_DISPLAY_NAME,
        }
    }
}
