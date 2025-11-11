/// # Spatial hash grid cell size
/// This is a fundamental lengthscale in the game.
/// It is the cell size in all spatial hash grids used in the world.
/// This means that no player can ever see further than this distance, that no spells can travel
/// further than this distance etc.
/// This is because queries over ranges larger than this will be incorrect.
///
/// To get more of an idea of how this works, imagine how the player's client needs to work.
/// When the player logs in, the client needs to request information from the server about
/// everything that the player might need to see/render.
/// On the backend, this will involve a spatial query over the player's vicinity.
/// If the player can see further than 50 units, then the backend will need to run queries that
/// involve more than just the player's cell + adjacent cells.
/// This is not supported because of the horrendous performance implications.
pub const SPATIAL_HASH_GRID_CELL_SIZE: f32 = 50.0;

pub const IDENTITY_PREFIX_CAPACITY: usize = 8;
pub const DISPLAY_NAME_CAPACITY: usize = 16;

/// # Inventory size
/// The size of a character's inventory.
/// Inspired by runescape.
/// Runescape gets really interesting when you're working around finite inventory space.
/// Runescape's inventory is 28 slots - I wanted to make a game that pushes inventory space even
/// further.
pub const INVENTORY_SIZE: usize = INVENTORY_ROWS * INVENTORY_COLUMNS;
pub const INVENTORY_ROWS: usize = 4;
pub const INVENTORY_COLUMNS: usize = 4;
