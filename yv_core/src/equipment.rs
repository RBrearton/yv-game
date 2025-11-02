use crate::prelude::*;

/// # Equipment
/// A struct that contains a character's equipment.
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub cloak: Option<equippables::Cloak>,
    pub belt: Option<equippables::Belt>,
    pub chest: Option<equippables::Chest>,
    pub finger_1: Option<equippables::Finger>,
    pub finger_2: Option<equippables::Finger>,
    pub finger_3: Option<equippables::Finger>,
    pub finger_4: Option<equippables::Finger>,
    pub footwear: Option<equippables::Footwear>,
    pub hands: Option<equippables::Hands>,
    pub helmet: Option<equippables::Helmet>,
    pub legs: Option<equippables::Legs>,
    pub pouch: Option<equippables::Pouch>,
    pub projectile: Option<equippables::Projectile>,
    pub main_hand: Option<equippables::Weapon>,
    pub off_hand: Option<equippables::Weapon>,
}

impl Equipment {
    /// # To array
    /// Converts the equipment to a fixed-size array of optional equippables.
    /// No heap allocations.
    pub fn to_array(&self) -> [Option<Equippable>; 15] {
        [
            self.cloak.map(Equippable::Cloak),
            self.belt.map(Equippable::Belt),
            self.chest.map(Equippable::Chest),
            self.finger_1.map(Equippable::Finger),
            self.finger_2.map(Equippable::Finger),
            self.finger_3.map(Equippable::Finger),
            self.finger_4.map(Equippable::Finger),
            self.footwear.map(Equippable::Footwear),
            self.hands.map(Equippable::Hands),
            self.helmet.map(Equippable::Helmet),
            self.legs.map(Equippable::Legs),
            self.pouch.map(Equippable::Pouch),
            self.projectile.map(Equippable::Projectile),
            self.main_hand.map(Equippable::Weapon),
            self.off_hand.map(Equippable::Weapon),
        ]
    }

    /// # Stats array
    /// A helper method for getting a fixed size array of stats, one for each equippable.
    pub fn stats_array(&self) -> [Stats; 15] {
        self.to_array().map(|item| item.stats())
    }
}

impl From<Equipment> for [Option<Equippable>; 15] {
    fn from(equipment: Equipment) -> Self {
        equipment.to_array()
    }
}

impl HasStats for Equipment {
    fn stats(&self) -> Stats {
        Stats::add(self.stats_array())
    }
}
