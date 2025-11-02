use crate::prelude::*;

/// # Mineable
/// The trait that must be implemented by anything that can be mined.
#[delegatable_trait]
pub trait Mineable {
    /// # Mining difficulty
    /// Returns the mining difficulty of the object.
    fn mining_difficulty(&self) -> u16;

    /// # Minimum mining level
    /// Returns the minimum mining level required to mine the object.
    fn minimum_mining_level(&self) -> u16;

    /// # Ore
    /// The ore that will be mined from the object upon successful mining.
    fn ore(&self) -> ores::Ore;
}
