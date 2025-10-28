use crate::prelude::*;

/// # Dagger
/// An enum containing all dagger slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Dagger {
    StoneDagger,
    IronDagger,
}
