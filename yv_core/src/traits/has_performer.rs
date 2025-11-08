use crate::prelude::*;

/// # Has performer
/// The trait that must be implemented by anything that has a performer.
#[delegatable_trait]
pub trait HasPerformer {
    /// # Performer
    /// Returns the performer of the object.
    fn performer(&self) -> &Identity;
}
