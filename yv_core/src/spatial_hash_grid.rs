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
    pub fn from_position(position: Vector2, cell_size: f32) -> Self {
        let x = (position.x / cell_size).floor() as i32;
        let y = (position.y / cell_size).floor() as i32;
        Self { x, y }
    }
}

/// # Spatial hash grid
/// A spatial hash grid is a data structure that allows for extremely fast spatial queries, when
/// the queries are restricted to a characteristic lengthscale, and when all queries are over
/// lengthscales that are lower than a certain value (the cell size).
pub struct SpatialHashGrid<T: ActorLike> {
    cells: HashMap<CellIndex, HashMap<Identity, T>>,
    cell_size: f32,
}

impl<T: ActorLike> SpatialHashGrid<T> {
    /// # New empty
    /// Creates a new empty spatial hash grid.
    pub fn new_empty(cell_size: f32) -> Self {
        Self {
            cells: HashMap::new(),
            cell_size,
        }
    }

    /// Returns an iterator over all the entities in the cell with the given index.
    fn get_entities_in_cell(&self, cell_index: CellIndex) -> impl Iterator<Item = &T> {
        self.cells
            .get(&cell_index)
            .into_iter()
            .flat_map(|cell| cell.values())
    }

    /// Given a position, returns the cell index that the position is in, as well as the eight
    /// adjacent cells.
    fn get_cells_near_position(&self, position: Vector2) -> [CellIndex; 9] {
        let cell_index = CellIndex::from_position(position, self.cell_size);
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
