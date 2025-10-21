use crate::traits::{HasPlacement, Identifiable};

/// # Actor like
/// This trait is implemented by all actors in the game.
/// An actor is the simplest kind of object that can exist in the game world.
/// In general, it's just something that we are capable of seeing.
pub trait ActorLike: HasPlacement + Identifiable {}
