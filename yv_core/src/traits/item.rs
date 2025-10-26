use crate::prelude::*;

/// # Item
/// This trait defines what we expect from all items in the game.
pub trait Item:
    Ownable
    + Describable
    + HasDisplayName
    + HasStats
    + Massive
    + Copy
    + Clone
    + Serialize
    + for<'a> Deserialize<'a>
{
    /// # Is stackable
    /// Whether this item can be stacked in an inventory, or not.
    /// Items that can stack can be stacked infinitely - the only bottleneck is the resultant mass.
    fn is_stackable() -> bool;
}
