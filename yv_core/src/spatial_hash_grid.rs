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
}

impl From<Vector2> for CellIndex {
    fn from(position: Vector2) -> Self {
        let x = (position.x / well_known_terms::SPATIAL_HASH_GRID_CELL_SIZE).floor() as i32;
        let y = (position.y / well_known_terms::SPATIAL_HASH_GRID_CELL_SIZE).floor() as i32;
        Self { x, y }
    }
}

/// # Spatial hash grid
/// A spatial hash grid is a data structure that allows for extremely fast spatial queries, when
/// the queries are restricted to a characteristic lengthscale, and when all queries are over
/// lengthscales that are lower than a certain value (the cell size).
pub struct SpatialHashGrid {
    id_to_pos: HashMap<Identity, Vector2>,
    idx_to_ids: HashMap<CellIndex, HashSet<Identity>>,
}

impl SpatialHashGrid {
    /// # New empty
    /// Creates a new empty spatial hash grid.
    pub fn new_empty() -> Self {
        Self {
            idx_to_ids: HashMap::new(),
            id_to_pos: HashMap::new(),
        }
    }

    /// # Add actor
    /// Adds an actor to the spatial hash grid.
    /// This is a convenience alias for the add method.
    pub fn add_actor(&mut self, actor: &impl ActorLike) {
        self.add(actor.identity(), actor.position().into());
    }

    /// Adds an (identity, position) pair to the spatial hash grid.
    pub fn add(&mut self, id: Identity, pos: Vector2) {
        self.id_to_pos.insert(id, pos);
        self.idx_to_ids.entry(pos.into()).or_default().insert(id);
    }

    /// Updates the position of an actor in the spatial hash grid.
    /// You should call this whenever the actor moves.
    pub fn update(&mut self, id: Identity, new_pos: Vector2) {
        self.remove(id);
        self.add(id, new_pos.into());
    }

    /// Get the identities of all entities that are within range of the requested position.
    /// These identities are added to a pre-prepared vec.
    pub fn get_in_range(&self, range: f32, query_pos: Vector2, out: &mut Vec<Identity>) {
        self.cells_near_position(query_pos)
            .iter()
            // Give me the id hashsets for every nonempty cell.
            .filter_map(|cell| self.idx_to_ids.get(cell))
            // Flatten these hashsets and handle each id one at a time.
            .flatten()
            // Get rid of bugged ids that don't have a position.
            // Grab the positions while we're at it.
            .filter_map(|id| {
                let maybe_pos = self.id_to_pos.get(id);
                match maybe_pos {
                    Some(pos) => Some((*id, *pos)),
                    None => None,
                }
            })
            // Run the range check - filter out ids that aren't within range.
            .filter(|(_id, pos)| query_pos.distance_to(*pos) <= range)
            // Add every id that passes the range check.
            .for_each(|(id, _pos)| out.push(id));
    }

    /// Removes an id from the spatial hash grid.
    /// This is infallible - it just won't do anything if the id isn't found.
    pub fn remove(&mut self, id: Identity) {
        let maybe_pos = self.id_to_pos.remove(&id);

        // Make sure that the id was actually tracked by the spatial hash grid.
        if let Some(pos) = maybe_pos {
            // Try to get the set of identities in the cell.
            let cell_idx: CellIndex = pos.into();
            let cell_set = self.idx_to_ids.get_mut(&cell_idx);

            // Make sure that the cell was actually tracked by the spatial hash grid.
            if let Some(cell_set) = cell_set {
                // Remove the identity from the set.
                cell_set.remove(&id);

                // If the cell is now empty, remove it from the cells map.
                if cell_set.is_empty() {
                    self.idx_to_ids.remove(&cell_idx);
                }
            }
        }
    }

    /// Returns whether the actor is in this spatial hash grid.
    pub fn contains(&self, id: &Identity) -> bool {
        self.id_to_pos.contains_key(id)
    }

    pub fn contains_actor(&self, actor: &impl ActorLike) -> bool {
        self.contains(&actor.identity())
    }

    /// Returns the number of items tracked in the grid.
    pub fn len(&self) -> usize {
        self.id_to_pos.len()
    }

    /// Given a position, returns the cell index that the position is in, as well as the eight
    /// adjacent cells.
    fn cells_near_position(&self, position: Vector2) -> [CellIndex; 9] {
        let cell_index: CellIndex = position.into();
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

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple test actor that implements all required traits
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
    struct TestActor {
        id: Identity,
        placement: Placement,
    }

    impl TestActor {
        fn new(id: Identity, x: f32, y: f32) -> Self {
            Self {
                id,
                placement: Placement {
                    position: Vector2 { x, y },
                    rotation: 0.0,
                },
            }
        }

        fn test_id() -> Identity {
            Identity::new(IdentityPrefix::new("test"))
        }
    }

    impl Identifiable for TestActor {
        fn identity(&self) -> Identity {
            self.id
        }
    }

    impl HasDisplayName for TestActor {
        fn display_name(&self) -> &str {
            "test_actor"
        }
    }

    impl HasPlacement for TestActor {
        fn placement(&self) -> &Placement {
            &self.placement
        }

        fn placement_mut(&mut self) -> &mut Placement {
            &mut self.placement
        }
    }

    impl ActorLike for TestActor {}

    #[test]
    fn test_new_empty_grid() {
        let grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 2., 3.);
        assert!(!grid.contains_actor(&actor));
        assert_eq!(0, grid.len());
    }

    #[test]
    fn test_add_multiple_actors_same_cell() {
        let mut grid = SpatialHashGrid::new_empty();
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();

        // Both actors in same cell (within 50 units).
        let actor1 = TestActor::new(id1, 10.0, 10.0);
        let actor2 = TestActor::new(id2, 15.0, 15.0);

        grid.add_actor(&actor1);
        grid.add_actor(&actor2);

        assert!(grid.contains_actor(&actor1));
        assert!(grid.contains_actor(&actor2));
    }

    #[test]
    fn test_add_actors_different_cells() {
        let mut grid = SpatialHashGrid::new_empty();
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();

        // Actors in different cells (cell size is 50.0)
        let actor1 = TestActor::new(id1, 10.0, 10.0);
        let actor2 = TestActor::new(id2, 100.0, 100.0);

        grid.add_actor(&actor1);
        grid.add_actor(&actor2);

        assert!(grid.contains_actor(&actor1));
        assert!(grid.contains_actor(&actor2));
    }

    #[test]
    fn test_remove() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 20.0);

        grid.add_actor(&actor);
        assert!(grid.contains_actor(&actor));

        grid.remove(id);
        assert!(!grid.contains_actor(&actor));
    }

    #[test]
    fn test_remove_nonexistent_actor() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();

        grid.remove(id); // Should not throw anything.
        assert!(!grid.contains(&id)); // Was never in to begin with.
    }

    #[test]
    fn test_update_position_same_cell() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 10.0);

        grid.add_actor(&actor);

        // Move within same cell
        let new_pos = Vector2 { x: 15.0, y: 15.0 };
        grid.update(id, new_pos);
        let new_grid_pos = grid.id_to_pos[&id];
        assert_eq!(new_pos, new_grid_pos);
        assert!(grid.contains(&id));
        assert_eq!(1, grid.len());
    }

    #[test]
    fn test_update_position_different_cell() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 10.0);

        grid.add_actor(&actor);

        // Move within same cell
        let new_pos = Vector2 {
            x: 1005.0,
            y: 1500.0,
        };
        grid.update(id, new_pos);
        let new_grid_pos = grid.id_to_pos[&id];
        assert_eq!(new_pos, new_grid_pos);
        assert!(grid.contains(&id));
        assert_eq!(1, grid.len());

        // This last one is subtle but an important implementation detail.
        // We should get rid of the now empty idx_to_ids entry in the hashmap.
        assert_eq!(1, grid.idx_to_ids.len());
    }

    #[test]
    fn test_get_in_range_empty_grid() {
        let grid = SpatialHashGrid::new_empty();
        let mut results = Vec::new();
        let query_pos = Vector2 { x: 0.0, y: 0.0 };

        grid.get_in_range(50.0, query_pos, &mut results);

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_in_range_single_actor_in_range() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 10.0);

        grid.add_actor(&actor);

        let mut results = Vec::new();
        let query_pos = Vector2 { x: 15.0, y: 15.0 };

        grid.get_in_range(10.0, query_pos, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0], id);
    }

    #[test]
    fn test_get_in_range_single_actor_out_of_range() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 10.0);

        grid.add_actor(&actor);

        let mut results = Vec::new();
        let query_pos = Vector2 { x: 100.0, y: 100.0 };

        grid.get_in_range(10.0, query_pos, &mut results);

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_in_range_multiple_actors() {
        let mut grid = SpatialHashGrid::new_empty();
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();
        let id3 = TestActor::test_id();

        grid.add_actor(&TestActor::new(id1, 0.0, 0.0));
        grid.add_actor(&TestActor::new(id2, 5.0, 0.0));
        grid.add_actor(&TestActor::new(id3, 100.0, 100.0));

        let mut ids = Vec::new();
        let query_pos = Vector2 { x: 0.0, y: 0.0 };

        grid.get_in_range(10.0, query_pos, &mut ids);

        // Should find id1 and id2, but not id3
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
        assert!(!ids.contains(&id3));
    }

    #[test]
    fn test_get_in_range_exact_boundary() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 0.0);

        grid.add_actor(&actor);

        let mut results = Vec::new();
        let query_pos = Vector2 { x: 0.0, y: 0.0 };

        // Actor is exactly 10.0 units away
        grid.get_in_range(10.0, query_pos, &mut results);
        assert_eq!(results.len(), 1);

        results.clear();
        grid.get_in_range(9.9, query_pos, &mut results);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_in_range_across_cell_boundaries() {
        let mut grid = SpatialHashGrid::new_empty();

        // Create actors near cell boundaries
        // Cell size is 50.0, so boundaries are at 0, 50, 100, etc.
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();
        let id3 = TestActor::test_id();

        grid.add_actor(&TestActor::new(id1, 48.0, 48.0)); // Near boundary
        grid.add_actor(&TestActor::new(id2, 52.0, 52.0)); // Just over boundary
        grid.add_actor(&TestActor::new(id3, 55.0, 55.0)); // Further in next cell

        let mut results = Vec::new();
        let query_pos = Vector2 { x: 50.0, y: 50.0 };

        grid.get_in_range(10.0, query_pos, &mut results);

        // Should find all three actors as they're all within ~10 units
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_negative_coordinates() {
        let mut grid = SpatialHashGrid::new_empty();
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();

        grid.add_actor(&TestActor::new(id1, -10.0, -10.0));
        grid.add_actor(&TestActor::new(id2, -15.0, -15.0));

        let mut results = Vec::new();
        let query_pos = Vector2 { x: -10.0, y: -10.0 };

        grid.get_in_range(10.0, query_pos, &mut results);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_cell_index_calculation() {
        // Test that CellIndex::from_position works correctly
        // Cell size is 50.0

        // Positive coordinates
        assert_eq!(
            CellIndex::from(Vector2 { x: 0.0, y: 0.0 }),
            CellIndex::new(0, 0)
        );
        assert_eq!(
            CellIndex::from(Vector2 { x: 25.0, y: 25.0 }),
            CellIndex::new(0, 0)
        );
        assert_eq!(
            CellIndex::from(Vector2 { x: 50.0, y: 50.0 }),
            CellIndex::new(1, 1)
        );
        assert_eq!(
            CellIndex::from(Vector2 { x: 99.0, y: 99.0 }),
            CellIndex::new(1, 1)
        );

        // Negative coordinates
        assert_eq!(
            CellIndex::from(Vector2 { x: -1.0, y: -1.0 }),
            CellIndex::new(-1, -1)
        );
        assert_eq!(
            CellIndex::from(Vector2 { x: -50.0, y: -50.0 }),
            CellIndex::new(-1, -1)
        );
    }
}
