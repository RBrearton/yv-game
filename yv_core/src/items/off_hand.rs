use crate::prelude::*;

/// # Off hand
/// An enum containing all off hand slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OffHand {
    Shield(Shield),
}

impl Describable for OffHand {
    fn description(&self) -> &str {
        match self {
            OffHand::Shield(shield) => shield.description(),
        }
    }
}
impl HasDisplayName for OffHand {
    fn display_name(&self) -> &str {
        match self {
            OffHand::Shield(shield) => shield.display_name(),
        }
    }
}
