use crate::prelude::*;

/// # Has target
/// The trait that must be implemented by anything that has a target.
#[delegatable_trait]
pub trait HasTarget {
    /// # Target
    /// The identity of the target of the action.
    /// If this is None, then the action is a self-targeting action. and the performer is the
    /// target.
    fn target(&self) -> Option<&Identity>;
}
