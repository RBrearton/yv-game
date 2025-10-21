use crate::prelude::*;

pub trait Identifiable {
    /// # Identity
    /// Returns the identity of the object.
    fn identity(&self) -> Identity;
}
