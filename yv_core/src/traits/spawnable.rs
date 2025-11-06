use crate::prelude::*;

/// # Spawnable
/// The trait that must be implemented by anything that can be dynamically spawned in the game
/// world.
pub trait Spawnable {
    /// # Spawn
    /// Spawns the object with the given placement.
    fn spawn(placement: Placement) -> Self;
}
