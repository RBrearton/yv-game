use crate::prelude::*;

/// # Projectile
/// An enum containing all projectile items in the game (really, these are just things that go in
/// a quiver).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Projectile {
    StoneArrow(equippables::quiver_slot::StoneArrow),
    IronArrow(equippables::quiver_slot::IronArrow),
    SteelArrow(equippables::quiver_slot::SteelArrow),
}
