use crate::prelude::*;

/// # Has requirements
/// The trait that must be implemented by anything that has requirements.
/// This could be anything from equipping an item to finishing a quest to starting an action.
#[delegatable_trait]
pub trait HasRequirements {
    /// # Requirements
    /// Returns the requirements of the object.
    ///
    /// Note that this is an owned instance, not a reference.
    /// This might seem weird, but actually is very important from a networking performance
    /// perspective.
    /// If we returned references to requirements, everything in the game that implements
    /// HasRequirements would need to track their requirements directly.
    /// That's every single Equippable - and the full Requirements struct is quite large.
    /// It makes the character INVENTORY_SIZE * 128 bytes larger.
    /// This actually adds a few kilobytes to every single character's memory footprint, which may
    /// be ~10x larger than it needs to be.
    fn requirements(&self) -> Requirements;
}
