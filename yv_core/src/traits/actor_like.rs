use crate::prelude::*;

/// # Actor like
/// This trait is implemented by all actors in the game.
/// An actor is the simplest kind of object that can exist in the game world.
/// In general, it's just something that we are capable of seeing.
#[delegatable_trait]
pub trait ActorLike:
    HasPlacement + HasDisplayName + Identifiable + Copy + Clone + Serialize + for<'a> Deserialize<'a>
{
}
