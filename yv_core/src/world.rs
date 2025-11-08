use crate::prelude::*;

/// # Environment
/// The environment consists of interactive objects that can't move.
/// This includes trees that we can chop down, rocks that we can mine, etc.
pub struct Environment {
    pub trees: SpatialHashGrid<environment::Tree>,
    pub mining_nodes: SpatialHashGrid<environment::MiningNode>,
}

impl Environment {
    /// # New empty
    /// Creates a new empty environment.
    pub fn new_empty() -> Self {
        Self {
            trees: SpatialHashGrid::new_empty(),
            mining_nodes: SpatialHashGrid::new_empty(),
        }
    }
}

/// # Characters
/// The data for all the characters in the game is stored here.
pub struct Characters {
    pub players: SpatialHashGrid<characters::Player>,
    pub npcs: SpatialHashGrid<characters::npcs::Npc>,
}
impl Characters {
    /// # New empty
    /// Creates a new empty characters.
    pub fn new_empty() -> Self {
        Self {
            players: SpatialHashGrid::new_empty(),
            npcs: SpatialHashGrid::new_empty(),
        }
    }
}

pub struct World {
    pub environment: Environment,
    pub characters: Characters,
}

impl World {
    /// # New empty
    /// Creates a new empty world.
    pub fn new_empty() -> Self {
        Self {
            environment: Environment::new_empty(),
            characters: Characters::new_empty(),
        }
    }
}
