use crate::prelude::*;

/// # Dual wield
/// A struct containing the weapons that a character is dual wielding.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DualWield {
    pub main_hand: Option<MainHandWieldable>,
    pub off_hand: Option<OffHandWieldable>,
}
