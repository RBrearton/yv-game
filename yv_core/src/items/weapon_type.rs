use crate::prelude::*;

/// # Weapon type
/// A simple enum representing how we can wield a weapon.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeaponType {
    MainHand,
    OffHand,
    TwoHand,
    Any,
}
