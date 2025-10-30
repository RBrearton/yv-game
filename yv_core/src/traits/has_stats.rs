use crate::prelude::*;

/// # Has stats
/// The trait that must be implemented by anything that has stats.
pub trait HasStats {
    /// # Stats
    /// Returns a reference to the stats of the object.
    fn stats(&self) -> &Stats;

    /// # Get stat
    /// Gets the stat with the given type.
    fn get_stat(&self, stat_type: StatType) -> Stat {
        self.stats().get_stat(stat_type)
    }
}
