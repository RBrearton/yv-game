/// # Environment
/// The environment consists of interactive objects that can't move.
/// This includes trees that we can chop down, rocks that we can mine, etc.
pub struct Environment {}

impl Environment {
    /// # New empty
    /// Creates a new empty environment.
    pub fn new_empty() -> Self {
        Self {}
    }
}
/// # Players
/// Stores all the players in the game.
pub struct Players {}

impl Players {
    /// # New empty
    /// Creates a new empty players.
    pub fn new_empty() -> Self {
        Self {}
    }
}
/// # NPCs
/// All NPCs in the game.
pub struct Npcs {}

impl Npcs {
    /// # New empty
    /// Creates a new empty NPCs.
    pub fn new_empty() -> Self {
        Self {}
    }
}
/// # World
/// The world is the main data structure that contains everything in the entire game world.
/// It holds the full game state.
/// This is the highest level data structure in the game.
pub struct World {
    environment: Environment,
    players: Players,
    npcs: Npcs,
}

impl World {
    /// # New empty
    /// Creates a new empty world.
    pub fn new_empty() -> Self {
        Self {
            environment: Environment::new_empty(),
            players: Players::new_empty(),
            npcs: Npcs::new_empty(),
        }
    }
}
