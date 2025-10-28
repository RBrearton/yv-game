use crate::prelude::*;

/// # Equipment
/// A struct that contains a character's equipment.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub helmet: Option<Helmet>,
    pub chest: Option<Chest>,
    pub legs: Option<Legs>,
    pub cloak: Option<Cloak>,
    pub one_hand: Option<OneHand>,
    pub off_hand: Option<OffHand>,
    pub two_hand: Option<TwoHand>,
}
