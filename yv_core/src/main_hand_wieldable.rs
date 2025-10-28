use crate::prelude::*;

/// # Main hand wieldable
/// An enum containing all items that can be wielded in the main hand.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum MainHandWieldable {
    OneHand(OneHand),
}
