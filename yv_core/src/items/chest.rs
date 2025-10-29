use crate::prelude::*;

const LINEN_TUNIC_DESCRIPTION: &str = "A simple linen tunic.";
const LEATHER_TUNIC_DESCRIPTION: &str = "A simple leather tunic.";

const LINEN_TUNIC_DISPLAY_NAME: &str = "Linen tunic";
const LEATHER_TUNIC_DISPLAY_NAME: &str = "Leather tunic";

/// # Chest
/// An enum containing all chest slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Chest {
    LinenTunic,
    LeatherTunic,
}

impl Describable for Chest {
    fn description(&self) -> &str {
        match self {
            Chest::LinenTunic => LINEN_TUNIC_DESCRIPTION,
            Chest::LeatherTunic => LEATHER_TUNIC_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Chest {
    fn display_name(&self) -> &str {
        match self {
            Chest::LinenTunic => LINEN_TUNIC_DISPLAY_NAME,
            Chest::LeatherTunic => LEATHER_TUNIC_DISPLAY_NAME,
        }
    }
}
