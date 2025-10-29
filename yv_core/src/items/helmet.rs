use crate::prelude::*;

const IRON_SKULLCAP_DESCRIPTION: &str = "An iron skullcap.";
const IRON_SKULLCAP_DISPLAY_NAME: &str = "Iron skullcap";

/// # Helmet
/// An enum containing all helmet slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Helmet {
    IronSkullcap,
}

impl Describable for Helmet {
    fn description(&self) -> &str {
        match self {
            Helmet::IronSkullcap => IRON_SKULLCAP_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Helmet {
    fn display_name(&self) -> &str {
        match self {
            Helmet::IronSkullcap => IRON_SKULLCAP_DISPLAY_NAME,
        }
    }
}
