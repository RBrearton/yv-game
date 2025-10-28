use crate::prelude::*;

/// # Equippable
/// An enum containing all equippable items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Equippable {
    Helmet(Helmet),
    Chest(Chest),
    Legs(Legs),
    Cloak(Cloak),
    OneHanded(OneHand),
    OffHand(OffHand),
    TwoHanded(TwoHand),
}
