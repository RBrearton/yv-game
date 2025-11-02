//! # Well known terms
//! This module contains some constants and terms that are used throughout the project.

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

pub mod augmentations {
    pub const SHARPENED: &str = "Sharpened";
    pub const LIGHT_REINFORCEMENT: &str = "Light reinforcement";
    pub const STRENGTHENED: &str = "Strengthened";
}
pub mod enchantments {
    pub const HASTE: &str = "Haste";
    pub const MINOR_HARVESTING: &str = "Minor harvesting";
}

pub mod quests {
    pub const TUTORIAL: &str = "Tutorial";
    pub const EXAMPLE_QUEST: &str = "Example quest";
}

pub mod skills {
    pub const STRENGTH: &str = "strength";
    pub const MINING: &str = "mining";
    pub const WOODCUTTING: &str = "woodcutting";
}

pub mod stats {
    pub const ARMOR: &str = "armour";
    pub const ATTACK_POWER: &str = "attack_power";
    pub const BLOCK_POINTS: &str = "block_points";
    pub const DURABILITY: &str = "durability";
    pub const SLASHING_BONUS: &str = "slashing_bonus";
    pub const STABBING_BONUS: &str = "stabbing_bonus";
    pub const CRUSHING_BONUS: &str = "crushing_bonus";
    pub const MINING_POWER: &str = "mining_power";
    pub const PRECISION: &str = "precision";
    pub const RANGED_ACCURACY: &str = "ranged_accuracy";
    pub const RANGED_DAMAGE: &str = "ranged_damage";
    pub const SPEED: &str = "speed";
    pub const VITALITY: &str = "vitality";
    pub const WARMTH: &str = "warmth";
    pub const WEAPON_SPEED: &str = "weapon_speed";
    pub const WOODCUTTING_POWER: &str = "woodcutting_power";
}

pub mod descriptions {
    pub mod augmentations {
        pub const SHARPENED: &str = "Sharpened weapons deal more slashing and stabbing damage.";
        pub const LIGHT_REINFORCEMENT: &str =
            "Lightly reinforced armour is slightly more resistant to physical damage, at the cost \
            of a small increase in weight.";
        pub const STRENGTHENED: &str =
            "Strengthened items don't break as easily, but are slightly heavier.";
    }
    pub mod enchantments {
        pub const HASTE: &str = "Substantially increases the wearer's speed.";
        pub const MINOR_HARVESTING: &str = "Improves the wearer's harvesting skills.";
    }

    pub mod quests {
        pub const TUTORIAL: &str = "The tutorial quest.";
        pub const EXAMPLE_QUEST: &str = "The example quest.";
    }

    pub mod skills {
        pub const STRENGTH: &str = "A measure of a character's physical strength.";
        pub const MINING: &str = "A measure of a character's mining skill.";
        pub const WOODCUTTING: &str = "A measure of a character's woodcutting skill.";
    }

    pub mod stats {
        pub const ARMOUR: &str =
            "The damage reduction applied by an object when the wearer is hit by an attack.";
        pub const ATTACK_POWER: &str =
            "A measure of how much additional damage the wearer will deal when using this object.";
        pub const DURABILITY: &str = "A measure of how long an object will last before breaking.";
        pub const SLASHING_BONUS: &str = "Additional attack power applied to slashing attacks.";
        pub const STABBING_BONUS: &str = "Additional attack power applied to stabbing attacks.";
        pub const CRUSHING_BONUS: &str = "Additional attack power applied to crushing attacks.";
        pub const BLOCK_POINTS: &str =
            "The damage reduction applied by an object when the wearer blocks an attack.";
        pub const MINING_POWER: &str =
            "A measure of how much ore an object can mine per unit of time.";
        pub const PRECISION: &str = "A measure of how accurate an object is.";
        pub const RANGED_ACCURACY: &str = "A measure of how accurate an object is at a distance.";
        pub const RANGED_DAMAGE: &str =
            "A measure of how much damage an object can deal at a distance.";
        pub const SPEED: &str = "A measure of how fast an object can move.";
        pub const VITALITY: &str = "A stat that increases the health of its wearer.";
        pub const WARMTH: &str = "A measure of how warm an object is.";
        pub const WEAPON_SPEED: &str = "A measure of how fast an object can attack.";
        pub const WOODCUTTING_POWER: &str =
            "A measure of how much wood an object can cut per unit of time.";
    }
}
