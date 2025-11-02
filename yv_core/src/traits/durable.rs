/// # Durable
/// The trait that must be implemented by anything that has durability.
pub trait Durable {
    /// # Current durability
    /// Returns the current durability of the object.
    fn current_durability(&self) -> u8;

    /// # Max durability
    /// Returns the max durability of the object.
    fn max_durability(&self) -> u8;

    /// # Decrease durability
    /// Decreases the durability of the object.
    fn decrease_durability(&mut self);

    /// # Restore durability
    /// Restores the durability of the object to the max durability.
    fn restore_durability(&mut self);

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
