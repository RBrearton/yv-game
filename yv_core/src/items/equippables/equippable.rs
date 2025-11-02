use crate::prelude::*;

/// # Equippable
/// An enum containing all equippable items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(HasStats)]
#[delegate(Durable)]
pub enum Equippable {
    Helmet(equippables::Helmet),
    Chest(equippables::Chest),
    Legs(equippables::Legs),
    Footwear(equippables::Footwear),
    Cloak(equippables::Cloak),
    Hands(equippables::Hands),
    Wrists(equippables::Wrists),
    Belt(equippables::Belt),
    Finger(equippables::Finger),
    Pouch(equippables::Pouch),
    Projectile(equippables::Projectile),
    Weapon(equippables::Weapon),
}

impl HasStats for Option<Equippable> {
    fn stats(&self) -> Stats {
        self.map(|item| item.stats()).unwrap_or_default()
    }
}
