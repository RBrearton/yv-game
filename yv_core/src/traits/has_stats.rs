use crate::prelude::*;

/// # Has stats
/// The trait that must be implemented by anything that has stats.
pub trait HasStats {
    /// # Stats
    ///
    /// Returns the stats of the object.
    /// Note that this is an owned instance, not a reference.
    /// This might seem weird, but actually is very important from a networking performance
    /// perspective.
    /// If we returned references to stats, everything in the game that implements HasStats would
    /// need to track their stats directly.
    /// That's every single equippable item - and the full Stats struct is quite large (88 bytes).
    /// For example, this means that every Item is a minimum of 88 bytes as Equippables are a minimum
    /// of 88 bytes.
    /// It makes the inventory INVENTORY_SIZE * 88 bytes larger, and the equipment 15 * 88 bytes
    /// larger.
    /// This actually adds a few kilobytes to every single character's memory footprint, which may
    /// be ~10x larger than it needs to be.
    ///
    /// Note that this doesn't prohibit a user from storing stats in an Equippable and returning a
    /// Copy/clone if necessary; it just means that we're free to make simpler, less memory-hungry
    /// alternatives if we want to.
    /// Naturally there's a minor performance hit due to copying lots of Stats instances around,
    /// but this is never going to bottleneck anything - stats calculations are never going to be
    /// something we're hammering hundreds of millions of times per second.
    fn stats(&self) -> Stats;

    /// # Get stat
    /// Gets the stat with the given type.
    fn get_stat(&self, stat_type: StatType) -> Stat {
        self.stats().get_stat(stat_type)
    }
}
