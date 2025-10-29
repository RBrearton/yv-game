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

impl Describable for Equippable {
    fn description(&self) -> &str {
        match self {
            Equippable::Helmet(helmet) => helmet.description(),
            Equippable::Chest(chest) => chest.description(),
            Equippable::Legs(legs) => legs.description(),
            Equippable::Cloak(cloak) => cloak.description(),
            Equippable::OneHanded(one_hand) => one_hand.description(),
            Equippable::OffHand(off_hand) => off_hand.description(),
            Equippable::TwoHanded(two_hand) => two_hand.description(),
        }
    }
}

impl HasDisplayName for Equippable {
    fn display_name(&self) -> &str {
        match self {
            Equippable::Helmet(helmet) => helmet.display_name(),
            Equippable::Chest(chest) => chest.display_name(),
            Equippable::Legs(legs) => legs.display_name(),
            Equippable::Cloak(cloak) => cloak.display_name(),
            Equippable::OneHanded(one_hand) => one_hand.display_name(),
            Equippable::OffHand(off_hand) => off_hand.display_name(),
            Equippable::TwoHanded(two_hand) => two_hand.display_name(),
        }
    }
}
