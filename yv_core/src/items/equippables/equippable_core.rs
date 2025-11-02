use crate::prelude::*;

/// # Equippable core
/// This is a core struct that contains common data that is likely to be shared by all equippables.
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquippableCore {
    pub augmentation: Option<Augmentation>,
    pub enchantment: Option<Enchantment>,
    pub imbuement: Option<Imbuement>,
    pub durability: Durability,
}

impl Durable for EquippableCore {
    fn current_durability(&self) -> u8 {
        self.durability.current_durability()
    }

    fn max_durability(&self) -> u8 {
        self.durability.max_durability()
    }

    fn decrease_durability(&mut self) {
        self.durability.decrease_durability()
    }

    fn restore_durability(&mut self) {
        self.durability.restore_durability()
    }
}
