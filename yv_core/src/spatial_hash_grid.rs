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
        let grid: SpatialHashGrid<TestActor> = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        assert!(grid.get_actor(id).is_none());
    }

    #[test]
    fn test_add_and_get_actor() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 20.0);

        grid.add_actor(actor);

        let retrieved = grid.get_actor(id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().position().x, 10.0);
        assert_eq!(retrieved.unwrap().position().y, 20.0);
    }

    #[test]
    fn test_add_multiple_actors_same_cell() {
        let mut grid = SpatialHashGrid::new_empty();
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();

        // Both actors in same cell (within 50 units)
        let actor1 = TestActor::new(id1, 10.0, 10.0);
        let actor2 = TestActor::new(id2, 15.0, 15.0);

        grid.add_actor(actor1);
        grid.add_actor(actor2);

        assert!(grid.get_actor(id1).is_some());
        assert!(grid.get_actor(id2).is_some());
    }

    #[test]
    fn test_add_actors_different_cells() {
        let mut grid = SpatialHashGrid::new_empty();
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();

        // Actors in different cells (cell size is 50.0)
        let actor1 = TestActor::new(id1, 10.0, 10.0);
        let actor2 = TestActor::new(id2, 100.0, 100.0);

        grid.add_actor(actor1);
        grid.add_actor(actor2);

        assert!(grid.get_actor(id1).is_some());
        assert!(grid.get_actor(id2).is_some());
    }

    #[test]
    fn test_get_actor_mut() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 20.0);

        grid.add_actor(actor);

        let actor_mut = grid.get_actor_mut(id);
        assert!(actor_mut.is_some());

        // Modify the actor directly (not recommended, use update_actor_position for position)
        actor_mut.unwrap().placement.rotation = std::f32::consts::PI;

        assert_eq!(grid.get_actor(id).unwrap().rotation(), std::f32::consts::PI);
    }

    #[test]
    fn test_remove_actor() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 20.0);

        grid.add_actor(actor);
        assert!(grid.get_actor(id).is_some());

        let removed = grid.remove_actor_with_id(id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().identity(), id);

        // Verify actor is gone
        assert!(grid.get_actor(id).is_none());
    }

    #[test]
    fn test_remove_nonexistent_actor() {
        let mut grid: SpatialHashGrid<TestActor> = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();

        let removed = grid.remove_actor_with_id(id);
        assert!(removed.is_none());
    }

    #[test]
    fn test_update_position_same_cell() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 10.0);

        grid.add_actor(actor);

        // Move within same cell
        let new_pos = Vector2 { x: 15.0, y: 15.0 };
        grid.update_actor_position(id, new_pos);

        let updated = grid.get_actor(id).unwrap();
        assert_eq!(updated.position().x, 15.0);
        assert_eq!(updated.position().y, 15.0);
    }

    #[test]
    fn test_update_position_different_cell() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 10.0);

        grid.add_actor(actor);

        // Move to different cell
        let new_pos = Vector2 { x: 100.0, y: 100.0 };
        grid.update_actor_position(id, new_pos);

        let updated = grid.get_actor(id).unwrap();
        assert_eq!(updated.position().x, 100.0);
        assert_eq!(updated.position().y, 100.0);

        // Verify actor is still findable via spatial query
        let mut results = Vec::new();
        grid.get_in_range(10.0, new_pos, &mut results);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].identity(), id);
    }

    #[test]
    fn test_update_position_across_multiple_cells() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 10.0);

        grid.add_actor(actor);

        // Move through several positions
        let positions = vec![
            Vector2 { x: 60.0, y: 10.0 }, // Different cell
            Vector2 { x: 60.0, y: 60.0 }, // Different cell
            Vector2 { x: 10.0, y: 60.0 }, // Different cell
            Vector2 { x: 10.0, y: 10.0 }, // Back to original
        ];

        for pos in positions {
            grid.update_actor_position(id, pos);
            let updated = grid.get_actor(id).unwrap();
            assert_eq!(updated.position(), pos);
        }
    }

    #[test]
    #[should_panic(expected = "actor should be tracked by the spatial hash grid")]
    fn test_update_nonexistent_actor_panics() {
        let mut grid: SpatialHashGrid<TestActor> = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let new_pos = Vector2 { x: 100.0, y: 100.0 };

        grid.update_actor_position(id, new_pos);
    }

    #[test]
    fn test_get_in_range_empty_grid() {
        let grid: SpatialHashGrid<TestActor> = SpatialHashGrid::new_empty();
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

        grid.add_actor(actor);

        let mut results = Vec::new();
        let query_pos = Vector2 { x: 15.0, y: 15.0 };

        grid.get_in_range(10.0, query_pos, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].identity(), id);
    }

    #[test]
    fn test_get_in_range_single_actor_out_of_range() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 10.0);

        grid.add_actor(actor);

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

        grid.add_actor(TestActor::new(id1, 0.0, 0.0));
        grid.add_actor(TestActor::new(id2, 5.0, 0.0));
        grid.add_actor(TestActor::new(id3, 100.0, 100.0));

        let mut results = Vec::new();
        let query_pos = Vector2 { x: 0.0, y: 0.0 };

        grid.get_in_range(10.0, query_pos, &mut results);

        // Should find id1 and id2, but not id3
        assert_eq!(results.len(), 2);
        let ids: Vec<Identity> = results.iter().map(|a| a.identity()).collect();
        assert!(ids.contains(&id1));
        assert!(ids.contains(&id2));
        assert!(!ids.contains(&id3));
    }

    #[test]
    fn test_get_in_range_exact_boundary() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        let actor = TestActor::new(id, 10.0, 0.0);

        grid.add_actor(actor);

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

        grid.add_actor(TestActor::new(id1, 48.0, 48.0)); // Near boundary
        grid.add_actor(TestActor::new(id2, 52.0, 52.0)); // Just over boundary
        grid.add_actor(TestActor::new(id3, 55.0, 55.0)); // Further in next cell

        let mut results = Vec::new();
        let query_pos = Vector2 { x: 50.0, y: 50.0 };

        grid.get_in_range(10.0, query_pos, &mut results);

        // Should find all three actors as they're all within ~10 units
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_get_in_range_with_removed_actors() {
        let mut grid = SpatialHashGrid::new_empty();
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();

        grid.add_actor(TestActor::new(id1, 10.0, 10.0));
        grid.add_actor(TestActor::new(id2, 15.0, 15.0));

        // Remove one actor
        grid.remove_actor_with_id(id1);

        let mut results = Vec::new();
        let query_pos = Vector2 { x: 10.0, y: 10.0 };

        grid.get_in_range(20.0, query_pos, &mut results);

        // Should only find id2
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].identity(), id2);
    }

    #[test]
    fn test_get_in_range_reuses_output_vector() {
        let mut grid = SpatialHashGrid::new_empty();
        let id = TestActor::test_id();
        grid.add_actor(TestActor::new(id, 10.0, 10.0));

        let mut results = Vec::with_capacity(100);
        let query_pos = Vector2 { x: 10.0, y: 10.0 };

        grid.get_in_range(10.0, query_pos, &mut results);
        assert_eq!(results.len(), 1);

        // Vector should be reusable (capacity maintained)
        let capacity_before = results.capacity();
        results.clear();
        grid.get_in_range(10.0, query_pos, &mut results);
        assert_eq!(results.capacity(), capacity_before);
    }

    #[test]
    fn test_negative_coordinates() {
        let mut grid = SpatialHashGrid::new_empty();
        let id1 = TestActor::test_id();
        let id2 = TestActor::test_id();

        grid.add_actor(TestActor::new(id1, -10.0, -10.0));
        grid.add_actor(TestActor::new(id2, -15.0, -15.0));

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
            CellIndex::from_position(Vector2 { x: 0.0, y: 0.0 }),
            CellIndex::new(0, 0)
        );
        assert_eq!(
            CellIndex::from_position(Vector2 { x: 25.0, y: 25.0 }),
            CellIndex::new(0, 0)
        );
        assert_eq!(
            CellIndex::from_position(Vector2 { x: 50.0, y: 50.0 }),
            CellIndex::new(1, 1)
        );
        assert_eq!(
            CellIndex::from_position(Vector2 { x: 99.0, y: 99.0 }),
            CellIndex::new(1, 1)
        );

        // Negative coordinates
        assert_eq!(
            CellIndex::from_position(Vector2 { x: -1.0, y: -1.0 }),
            CellIndex::new(-1, -1)
        );
        assert_eq!(
            CellIndex::from_position(Vector2 { x: -50.0, y: -50.0 }),
            CellIndex::new(-1, -1)
        );
    }

    #[test]
    fn test_stress_many_actors() {
        let mut grid = SpatialHashGrid::new_empty();
        let mut ids = Vec::new();

        // Add 1000 actors in a grid pattern
        for i in 0..1000 {
            let id = TestActor::test_id();
            ids.push(id);
            let x = (i % 100) as f32 * 10.0;
            let y = (i / 100) as f32 * 10.0;
            grid.add_actor(TestActor::new(id, x, y));
        }

        // Verify all actors are retrievable
        for id in &ids {
            assert!(grid.get_actor(*id).is_some());
        }

        // Perform spatial query
        let mut results = Vec::new();
        grid.get_in_range(50.0, Vector2 { x: 500.0, y: 50.0 }, &mut results);
        assert!(!results.is_empty());

        // Remove half the actors
        for id in ids.iter().take(500) {
            assert!(grid.remove_actor_with_id(*id).is_some());
        }

        // Verify removed actors are gone and remaining actors are still there
        for (i, id) in ids.iter().enumerate() {
            if i < 500 {
                assert!(grid.get_actor(*id).is_none());
            } else {
                assert!(grid.get_actor(*id).is_some());
            }
        }
    }
}
