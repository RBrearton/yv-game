use crate::prelude::*;

/// # Off hand wieldable
/// An enum containing all items that can be wielded in the off hand.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum OffHandWieldable {
    OneHand(OneHand),
    OffHand(OffHand),
}
