use crate::prelude::*;

/// # Durability
/// A struct that contains the durability of an equippable item.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Durability {
    durability: u8,
    max_durability: u8,
}

impl Default for Durability {
    fn default() -> Self {
        Self {
            durability: 20,
            max_durability: 20,
        }
    }
}

impl Durability {
    /// # New durability
    /// Creates a new durability with the given durability and max durability.
    pub fn new(durability: u8, max_durability: u8) -> Self {
        Self {
            durability,
            max_durability,
        }
    }
}

impl Durable for Durability {
    fn current_durability(&self) -> u8 {
        self.durability
    }

    fn max_durability(&self) -> u8 {
        self.max_durability
    }

    fn decrease_durability(&mut self) {
        self.durability = self.durability.saturating_sub(1);
    }

    fn restore_durability(&mut self) {
        self.durability = self.max_durability;
    }
}
