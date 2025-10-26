use crate::prelude::*;

/// # Cell index
/// A cell index is a 2D index that uniquely specifies a cell in the spatial hash grid.
/// This is not public - it is an implementation detail of the SpatialHashGrid struct.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct CellIndex {
    x: i32,
    y: i32,
}
impl CellIndex {
    /// # New
    /// Creates a new cell index with the given x and y coordinates.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// # From position
    /// Creates a new cell index from a position and a cell size.
    pub fn from_position(position: Vector2) -> Self {
        let x = (position.x / well_known_terms::SPATIAL_HASH_GRID_CELL_SIZE).floor() as i32;
        let y = (position.y / well_known_terms::SPATIAL_HASH_GRID_CELL_SIZE).floor() as i32;
        Self { x, y }
    }
}

/// # Spatial hash grid
/// A spatial hash grid is a data structure that allows for extremely fast spatial queries, when
/// the queries are restricted to a characteristic lengthscale, and when all queries are over
/// lengthscales that are lower than a certain value (the cell size).
/// The SpatialHashGrid owns all the actors that are in it - it is the source of truth for all
/// actors of any kind in the game.
pub struct SpatialHashGrid<T: ActorLike> {
    actors: HashMap<Identity, T>,
    actor_indices: HashMap<Identity, CellIndex>,
    cells: HashMap<CellIndex, HashSet<Identity>>,
}

impl<T: ActorLike> SpatialHashGrid<T> {
    /// # New empty
    /// Creates a new empty spatial hash grid.
    pub fn new_empty() -> Self {
        Self {
            cells: HashMap::new(),
            actors: HashMap::new(),
            actor_indices: HashMap::new(),
        }
    }

    /// # Add actor
    /// Adds an actor to the spatial hash grid.
    pub fn add_actor(&mut self, actor: T) {
        let id = actor.identity();
        let cell_idx = CellIndex::from_position(actor.position());
        self.actors.insert(id, actor);
        self.add_id_to_cell(id, cell_idx);
    }

    /// # Get actor
    /// Returns an actor from the spatial hash grid by its identity.
    pub fn get_actor(&self, id: Identity) -> Option<&T> {
        self.actors.get(&id)
    }

    /// # Get actor mut
    /// Returns a mutable reference to an actor from the spatial hash grid by its identity.
    pub fn get_actor_mut(&mut self, id: Identity) -> Option<&mut T> {
        self.actors.get_mut(&id)
    }

    /// # Update actor position
    /// Updates the position of an actor in the spatial hash grid.
    /// This will also update the actor's position in the game world irrespective of whether the
    /// cell changed.
    pub fn update_actor_position(&mut self, id: Identity, position: Vector2) {
        // Get a reference to the actor and update its position.
        let actor = self
            .get_actor_mut(id)
            .expect("actor should be tracked by the spatial hash grid when we update its position");
        actor.set_position(position);

        // Get the new and old cell indices.
        let new_cell_idx = CellIndex::from_position(position);
        let old_cell_idx = *self
            .actor_indices
            .get(&id)
            .expect("actor should be tracked by the spatial hash grid when we update its position");

        // If the actor cell changed, remove it from the old cell and add it to the new cell.
        if old_cell_idx != new_cell_idx {
            self.remove_id_from_cell(id, old_cell_idx);
            self.add_id_to_cell(id, new_cell_idx);
        }
    }

    /// # Remove actor with id
    /// Removes an actor from the spatial hash grid by its identity.
    pub fn remove_actor_with_id(&mut self, id: Identity) -> Option<T> {
        let cell_idx = self.actor_indices.remove(&id);

        // Make sure that the id was actually tracked by the spatial hash grid.
        if let Some(cell_idx) = cell_idx {
            self.remove_id_from_cell(id, cell_idx);
            self.actors.remove(&id)
        } else {
            // We didn't find the id in the spatial hash grid, so we return None.
            None
        }
    }

    /// # Add id to cell
    /// Associates an actor with a cell by adding its identity to the cell's set of identities.
    fn add_id_to_cell(&mut self, id: Identity, cell_idx: CellIndex) {
        self.actor_indices.insert(id, cell_idx);
        self.cells.entry(cell_idx).or_default().insert(id);
    }

    /// # Remove id from cell
    /// Removes an identity from its current cell.
    fn remove_id_from_cell(&mut self, id: Identity, cell_idx: CellIndex) {
        // Try to get the set of identities in the cell.
        let cell_set = self.cells.get_mut(&cell_idx);

        // Make sure that the cell was actually tracked by the spatial hash grid.
        if let Some(cell_set) = cell_set {
            // Remove the identity from the set.
            cell_set.remove(&id);

            // If the cell is now empty, remove it from the cells map.
            if cell_set.is_empty() {
                self.cells.remove(&cell_idx);
            }
        }
    }

    /// Returns an iterator over all the entities in the cell with the given index.
    fn get_entities_in_cell(&self, cell_index: CellIndex) -> impl Iterator<Item = &T> {
        let cell_iter = self.cells.get(&cell_index).into_iter().flatten();
        cell_iter.map(|identity| &self.actors[identity])
    }

    /// Given a position, returns the cell index that the position is in, as well as the eight
    /// adjacent cells.
    fn get_cells_near_position(&self, position: Vector2) -> [CellIndex; 9] {
        let cell_index = CellIndex::from_position(position);
        let mut cells = [cell_index; 9];
        for i in 0..3 {
            for j in 0..3 {
                let cell_x = cell_index.x + i as i32 - 1;
                let cell_y = cell_index.y + j as i32 - 1;
                cells[i * 3 + j] = CellIndex::new(cell_x, cell_y);
            }
        }
        cells
    }
}

impl<T: ActorLike> SpatialQueryEngine for SpatialHashGrid<T> {
    type EntityType = T;

    fn get_in_range(&self, range: f32, position: Vector2, out: &mut Vec<Self::EntityType>) {
        let cells = self.get_cells_near_position(position);
        for cell in cells {
            for entity in self.get_entities_in_cell(cell) {
                if entity.distance_to(position) <= range {
                    out.push(*entity);
                }
            }
        }
    }
}
