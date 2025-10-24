use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// # Stat
/// A stat is a single value that characterizes a particular aspect of an object.
/// For example, a character's strength, or a weapon's attack power.
/// Stats are used to calculate the overall effectiveness of an object in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    /// The type of stat.
    pub stat_type: StatType,

    /// The value of the stat.
    pub value: f32,
}

impl Stat {
    /// # New stat
    /// Creates a new stat with the given type and value.
    pub fn new(stat_type: StatType, value: f32) -> Self {
        Self { stat_type, value }
    }
}

impl HasDisplayName for Stat {
    fn display_name(&self) -> &str {
        self.stat_type.display_name()
    }
}
impl Describable for Stat {
    fn description(&self) -> &str {
        self.stat_type.description()
    }
}
