use crate::prelude::*;

/// # One hand
/// An enum containing all one hand slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OneHand {
    Dagger(Dagger),
    Hatchet(Hatchet),
    Pickaxe(Pickaxe),
}
