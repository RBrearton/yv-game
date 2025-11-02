use crate::prelude::*;

/// # Durable
/// The trait that must be implemented by anything that has durability.
pub trait Durable {
    /// # Durability
    /// Returns the durability of the object.
    fn durability(&self) -> &Durability;

    /// # Durability mut
    /// Returns a mutable reference to the durability of the object.
    fn durability_mut(&mut self) -> &mut Durability;

    /// # Current durability
    /// Returns the current durability of the object.
    fn current_durability(&self) -> u8 {
        self.durability().current_durability()
    }

    /// # Max durability
    /// Returns the max durability of the object.
    fn max_durability(&self) -> u8 {
        self.durability().max_durability()
    }

    /// # Decrease durability
    /// Decreases the durability of the object.
    fn decrease_durability(&mut self) {
        self.durability_mut().decrease_durability()
    }

    /// # Restore durability
    /// Restores the durability of the object to the max durability.
    fn restore_durability(&mut self) {
        self.durability_mut().restore_durability()
    }

    /// # Is broken
    /// Returns true if the durability is 0.
    fn is_broken(&self) -> bool {
        self.current_durability() == 0
    }

    /// # Is at max durability
    /// Returns true if the durability is equal to the max durability.
    fn is_at_max_durability(&self) -> bool {
        self.current_durability() == self.max_durability()
    }
}
