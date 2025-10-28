use crate::prelude::*;

/// # Item
/// An enum containing all items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Equippable(Equippable),
}
