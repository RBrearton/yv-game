use crate::prelude::*;

/// # Two hand
/// An enum containing all two hand slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TwoHand {
    Greatsword(Greatsword),
    Battleaxe(Battleaxe),
}
