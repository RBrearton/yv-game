use crate::prelude::*;

pub struct World {
    pub actors: HashMap<Identity, crate::actors::Actor>,
    pub hash_grid: SpatialHashGrid,
}

impl World {
    /// # New empty
    /// Creates a new empty world.
    pub fn new_empty() -> Self {
        Self {
            actors: HashMap::new(),
            hash_grid: SpatialHashGrid::new_empty(),
        }
    }
}
