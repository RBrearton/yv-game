use crate::prelude::*;

/// # Weapon
/// An enum containing all weapons in the game.
/// A weapon is defined as something that we equip in our hands.
/// We do this either by equipping it in our main hand, our off hand, or by two-handing it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Weapon {
    Dagger(Dagger),
    Hammer(Hammer),
    Hatchet(Hatchet),
    Pickaxe(Pickaxe),
    Shortsword(Shortsword),
    Shield(Shield),
    Greatsword(Greatsword),
    Battleaxe(Battleaxe),
}

impl Weapon {
    pub fn weapon_type(&self) -> WeaponType {
        match self {
            Weapon::Dagger(_) => WeaponType::Any,
            Weapon::Hammer(hammer) => hammer.weapon_type(),
            Weapon::Hatchet(hatchet) => hatchet.weapon_type(),
            Weapon::Pickaxe(pickaxe) => pickaxe.weapon_type(),
            Weapon::Shortsword(shortsword) => shortsword.weapon_type(),
            Weapon::Shield(_) => WeaponType::OffHand,
            Weapon::Greatsword(_) => WeaponType::TwoHand,
            Weapon::Battleaxe(_) => WeaponType::TwoHand,
        }
    }
}

impl Describable for Weapon {
    fn description(&self) -> &str {
        match self {
            Weapon::Dagger(dagger) => dagger.description(),
            Weapon::Hammer(hammer) => hammer.description(),
            Weapon::Hatchet(hatchet) => hatchet.description(),
            Weapon::Pickaxe(pickaxe) => pickaxe.description(),
            Weapon::Shortsword(shortsword) => shortsword.description(),
            Weapon::Shield(shield) => shield.description(),
            Weapon::Greatsword(greatsword) => greatsword.description(),
            Weapon::Battleaxe(battleaxe) => battleaxe.description(),
        }
    }
}
impl HasDisplayName for Weapon {
    fn display_name(&self) -> &str {
        match self {
            Weapon::Dagger(dagger) => dagger.display_name(),
            Weapon::Hammer(hammer) => hammer.display_name(),
            Weapon::Hatchet(hatchet) => hatchet.display_name(),
            Weapon::Pickaxe(pickaxe) => pickaxe.display_name(),
            Weapon::Shortsword(shortsword) => shortsword.display_name(),
            Weapon::Shield(shield) => shield.display_name(),
            Weapon::Greatsword(greatsword) => greatsword.display_name(),
            Weapon::Battleaxe(battleaxe) => battleaxe.display_name(),
        }
    }
}
impl HasStats for Weapon {
    fn stats(&self) -> &Stats {
        match self {
            Weapon::Dagger(dagger) => dagger.stats(),
            Weapon::Hammer(hammer) => hammer.stats(),
            Weapon::Hatchet(hatchet) => hatchet.stats(),
            Weapon::Pickaxe(pickaxe) => pickaxe.stats(),
            Weapon::Shortsword(shortsword) => shortsword.stats(),
            Weapon::Shield(shield) => shield.stats(),
            Weapon::Greatsword(greatsword) => greatsword.stats(),
            Weapon::Battleaxe(battleaxe) => battleaxe.stats(),
        }
    }
}
