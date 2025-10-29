use crate::prelude::*;

/// # Item
/// An enum containing all items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Equippable(Equippable),
}

impl Describable for Item {
    fn description(&self) -> &str {
        match self {
            Item::Equippable(equippable) => equippable.description(),
        }
    }
}
impl HasDisplayName for Item {
    fn display_name(&self) -> &str {
        match self {
            Item::Equippable(equippable) => equippable.display_name(),
        }
    }
}
