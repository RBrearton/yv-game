use crate::prelude::*;

/// # Equipment slot
/// An enum representing the different equipment slots that are available in the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Helmet,
    Chest,
    Legs,
    Back,
    TwoHanded,
    OneHanded,
    MainHand,
    OffHand,
}
