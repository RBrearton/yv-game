use crate::prelude::*;

/// # Chest
/// An enum containing all chest slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Chest {
    LinenTunic,
    LeatherTunic,
}
