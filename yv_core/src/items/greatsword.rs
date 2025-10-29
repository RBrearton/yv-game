use crate::prelude::*;

const IRON_GREATSWORD_DESCRIPTION: &str = "A greatsword made of iron.";
const IRON_GREATSWORD_DISPLAY_NAME: &str = "Iron greatsword";

/// # Greatsword
/// An enum containing all greatsword slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Greatsword {
    IronGreatsword,
}

impl Describable for Greatsword {
    fn description(&self) -> &str {
        match self {
            Greatsword::IronGreatsword => IRON_GREATSWORD_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Greatsword {
    fn display_name(&self) -> &str {
        match self {
            Greatsword::IronGreatsword => IRON_GREATSWORD_DISPLAY_NAME,
        }
    }
}
