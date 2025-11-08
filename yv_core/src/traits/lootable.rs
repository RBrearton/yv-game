use crate::prelude::*;

/// # Lootable
/// The trait that must be implemented by anything that can be looted.
#[delegatable_trait]
pub trait Lootable {
    /// # Loot
    /// Returns the loot of the object.
    /// This is the inventory of items that the looter will be able to pick up from the object.
    fn loot(&self) -> &Inventory;

    /// # Loot mut
    /// Returns a mutable reference to the loot.
    /// Downstream mechanisms may use this to add or remove items from the loot.
    fn loot_mut(&mut self) -> &mut Inventory;

    /// # Looter
    /// Returns the identity of the character that is permitted to loot the object.
    fn looter(&self) -> &Identity;

    /// # Set looter
    /// Sets the identity of the character that is permitted to loot the object.
    fn set_looter(&mut self, looter: Identity);
}
