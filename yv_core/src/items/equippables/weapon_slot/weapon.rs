use crate::prelude::*;

/// # Weapon
/// An enum containing all weapons in the game.
/// A weapon is defined as something that we equip in our hands.
/// We do this either by equipping it in our main hand, our off hand, or by two-handing it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Weapon {
    Dagger(equippables::weapon_slot::Dagger),
    Hammer(equippables::weapon_slot::Hammer),
    Hatchet(equippables::weapon_slot::Hatchet),
    Pickaxe(equippables::weapon_slot::Pickaxe),
    Shortsword(equippables::weapon_slot::Shortsword),
    Shield(equippables::weapon_slot::Shield),
    Greatsword(equippables::weapon_slot::Greatsword),
    Battleaxe(equippables::weapon_slot::Battleaxe),
}
