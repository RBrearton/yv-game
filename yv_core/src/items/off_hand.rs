use crate::prelude::*;

/// # Off hand
/// An enum containing all off hand slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OffHand {
    Shield(Shield),
}
