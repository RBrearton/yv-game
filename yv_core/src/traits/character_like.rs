use crate::prelude::*;

/// # Character like
/// The trait that needs to be implemented by anything that wants to be treated as a character in
/// the game.
/// This includes all player characters, as well as all non-player characters.
pub trait CharacterLike: ActorLike + HasStats {}
