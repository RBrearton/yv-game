use crate::prelude::*;

const RAGGED_CLOAK_DESCRIPTION: &str = "A ragged cloak made of cloth.";
const RAGGED_CLOAK_DISPLAY_NAME: &str = "Ragged cloak";

/// # Cloak
/// An enum containing all cloak slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Cloak {
    RaggedCloak,
}

impl Describable for Cloak {
    fn description(&self) -> &str {
        match self {
            Cloak::RaggedCloak => RAGGED_CLOAK_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Cloak {
    fn display_name(&self) -> &str {
        match self {
            Cloak::RaggedCloak => RAGGED_CLOAK_DISPLAY_NAME,
        }
    }
}
