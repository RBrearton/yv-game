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

    /// # Get durability
    /// Returns the durability of the equippable item.
    pub fn durability(&self) -> u8 {
        self.durability
    }

    /// # Get max durability
    /// Returns the max durability of the equippable item.
    pub fn max_durability(&self) -> u8 {
        self.max_durability
    }

    /// # Decrease
    /// Decrements the durability of the equippable item.
    pub fn decrease(&mut self) {
        self.durability = self.durability.saturating_sub(1);
    }

    /// # Is broken
    /// Returns true if the durability is 0.
    pub fn is_broken(&self) -> bool {
        self.durability == 0
    }

    /// # Is at max durability
    /// Returns true if the durability is equal to the max durability.
    pub fn is_at_max_durability(&self) -> bool {
        self.durability == self.max_durability
    }

    /// # Restore
    /// Restores the durability of the equippable item to the max durability.
    pub fn restore(&mut self) {
        self.durability = self.max_durability;
    }
}
