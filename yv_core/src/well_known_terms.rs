//! # Well known terms
//! This module contains some constants and terms that are used throughout the project.

pub mod stats {
    pub const ARMOR: &str = "armour";
    pub const ATTACK_POWER: &str = "attack_power";
    pub const BLOCK_POINTS: &str = "block_points";
    pub const MINING_POWER: &str = "mining_power";
    pub const PRECISION: &str = "precision";
    pub const RANGED_ACCURACY: &str = "ranged_accuracy";
    pub const RANGED_DAMAGE: &str = "ranged_damage";
    pub const SPEED: &str = "speed";
    pub const WARMTH: &str = "warmth";
    pub const WEAPON_SPEED: &str = "weapon_speed";
    pub const WOODCUTTING_POWER: &str = "woodcutting_power";
}

pub mod descriptions {
    pub mod stats {
        pub const ARMOUR: &str =
            "The damage reduction applied by an object when the wearer is hit by an attack.";
        pub const ATTACK_POWER: &str =
            "A measure of how much additional damage the wearer will deal when using this object.";
        pub const BLOCK_POINTS: &str =
            "The damage reduction applied by an object when the wearer blocks an attack.";
        pub const MINING_POWER: &str =
            "A measure of how much ore an object can mine per unit of time.";
        pub const PRECISION: &str = "A measure of how accurate an object is.";
        pub const RANGED_ACCURACY: &str = "A measure of how accurate an object is at a distance.";
        pub const RANGED_DAMAGE: &str =
            "A measure of how much damage an object can deal at a distance.";
        pub const SPEED: &str = "A measure of how fast an object can move.";
        pub const WARMTH: &str = "A measure of how warm an object is.";
        pub const WEAPON_SPEED: &str = "A measure of how fast an object can attack.";
        pub const WOODCUTTING_POWER: &str =
            "A measure of how much wood an object can cut per unit of time.";
    }
}
