use crate::prelude::*;

/// # Choppable
/// The trait that must be implemented by any tree that can be chopped down as part of the
/// woodcutting skill.
#[delegatable_trait]
pub trait Choppable {
    /// # Woodcutting difficulty
    /// Returns the woodcutting difficulty of the tree.
    fn woodcutting_difficulty(&self) -> u16;

    /// # Minimum woodcutting level
    /// Returns the minimum woodcutting level required to chop down the tree.
    fn minimum_woodcutting_level(&self) -> u16;

    /// # Wood
    /// The wood that will be obtained from the tree upon successful chopping.
    fn wood(&self) -> wood::Log;
}
