use crate::prelude::*;

/// # Equippable
/// An enum containing all equippable items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

impl HasStats for Equippable {
    fn stats(&self) -> &Stats {
        match self {
            Equippable::Helmet(helmet) => helmet.stats(),
            Equippable::Chest(chest) => chest.stats(),
            Equippable::Legs(legs) => legs.stats(),
            Equippable::Footwear(feet) => feet.stats(),
            Equippable::Cloak(cloak) => cloak.stats(),
            Equippable::Hands(hands) => hands.stats(),
            Equippable::Wrists(wrists) => wrists.stats(),
            Equippable::Belt(belt) => belt.stats(),
            Equippable::Finger(finger) => finger.stats(),
            Equippable::Pouch(pouch) => pouch.stats(),
            Equippable::Projectile(quiver) => quiver.stats(),
            Equippable::Weapon(weapon) => weapon.stats(),
        }
    }

    fn get_stat(&self, stat_type: StatType) -> Stat {
        self.stats().get_stat(stat_type)
    }
}

impl Describable for Equippable {
    fn description(&self) -> &str {
        match self {
            Equippable::Helmet(helmet) => helmet.description(),
            Equippable::Chest(chest) => chest.description(),
            Equippable::Legs(legs) => legs.description(),
            Equippable::Footwear(feet) => feet.description(),
            Equippable::Cloak(cloak) => cloak.description(),
            Equippable::Hands(hands) => hands.description(),
            Equippable::Wrists(wrists) => wrists.description(),
            Equippable::Belt(belt) => belt.description(),
            Equippable::Finger(finger) => finger.description(),
            Equippable::Pouch(pouch) => pouch.description(),
            Equippable::Projectile(quiver) => quiver.description(),
            Equippable::Weapon(weapon) => weapon.description(),
        }
    }
}

impl HasDisplayName for Equippable {
    fn display_name(&self) -> &str {
        match self {
            Equippable::Helmet(helmet) => helmet.display_name(),
            Equippable::Chest(chest) => chest.display_name(),
            Equippable::Legs(legs) => legs.display_name(),
            Equippable::Footwear(feet) => feet.display_name(),
            Equippable::Cloak(cloak) => cloak.display_name(),
            Equippable::Hands(hands) => hands.display_name(),
            Equippable::Wrists(wrists) => wrists.display_name(),
            Equippable::Belt(belt) => belt.display_name(),
            Equippable::Finger(finger) => finger.display_name(),
            Equippable::Pouch(pouch) => pouch.display_name(),
            Equippable::Projectile(quiver) => quiver.display_name(),
            Equippable::Weapon(weapon) => weapon.display_name(),
        }
    }
}
