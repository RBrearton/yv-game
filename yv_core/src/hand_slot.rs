use crate::prelude::*;

/// # Hand slot
/// An enum representing the different ways that we can fill our hand-based equipment slot(s).
/// This makes it easy to have two handed weapons, dual wielded weapons, etc.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum HandSlot {
    TwoHand(TwoHand),
    DualWield(DualWield),
}
