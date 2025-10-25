use crate::prelude::*;

/// # Ownable
/// The trait that must be satisfied by anything that can be owned.
pub trait Ownable {
    /// # Owner
    /// Returns the owner of the object, or None if it is not currently owned.
    fn owner(&self) -> Option<Identity>;

    /// # Set owner
    /// Sets the owner of the object.
    fn set_owner(&mut self, owner: Identity);

    /// # Clear owner
    /// Clears the owner of the object, making it unowned.
    fn clear_owner(&mut self);
}
