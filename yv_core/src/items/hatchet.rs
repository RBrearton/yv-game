use crate::prelude::*;

const IRON_HATCHET_DESCRIPTION: &str = "An iron hatchet.";
const IRON_HATCHET_DISPLAY_NAME: &str = "Iron hatchet";

/// # Hatchet
/// An enum containing all hatchet slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Hatchet {
    IronHatchet,
}

impl Describable for Hatchet {
    fn description(&self) -> &str {
        match self {
            Hatchet::IronHatchet => IRON_HATCHET_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Hatchet {
    fn display_name(&self) -> &str {
        match self {
            Hatchet::IronHatchet => IRON_HATCHET_DISPLAY_NAME,
        }
    }
}
