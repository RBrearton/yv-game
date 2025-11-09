use crate::prelude::*;

/// # Has requirements
/// The trait that must be implemented by anything that has requirements.
/// This could be anything from equipping an item to finishing a quest to starting an action.
#[delegatable_trait]
pub trait HasRequirements {
    /// # Requirements
    /// Returns a reference to the requirements of the object.
    /// This is generally a reference to a const requirements struct, as requirements normally are
    /// constant.
    fn requirements(&self) -> &Requirements;
}
