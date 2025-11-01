use crate::prelude::*;

/// # Equipment
/// A struct that contains a character's equipment.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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
    pub weapons: Option<HandSlot>,
}
