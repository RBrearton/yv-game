use crate::prelude::*;

const WOODEN_BUCKLER_DESCRIPTION: &str = "A wooden buckler.";
const WOODEN_BUCKLER_DISPLAY_NAME: &str = "Wooden buckler";

/// # Shield
/// An enum containing all shield slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shield {
    WoodenBuckler,
}

impl Describable for Shield {
    fn description(&self) -> &str {
        match self {
            Shield::WoodenBuckler => WOODEN_BUCKLER_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Shield {
    fn display_name(&self) -> &str {
        match self {
            Shield::WoodenBuckler => WOODEN_BUCKLER_DISPLAY_NAME,
        }
    }
}
