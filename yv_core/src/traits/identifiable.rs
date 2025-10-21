use crate::prelude::*;

/// # Identifiable
/// The trait that must be implemented by anything that needs to be uniquely identified.
pub trait Identifiable {
    /// # Identity
    /// Returns the identity of the object.
    fn identity(&self) -> Identity;
}
