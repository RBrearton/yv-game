//! # Well known terms
//! This module contains some constants and terms that are used throughout the project.

const TERRAIN_SAND_EPSILON: f32 = 0.05;
pub const TERRAIN_CHUNK_SIZE: f32 = 2.0;
pub const TERRAIN_CONFIG_SEED: u32 = 42;
pub const TERRAIN_WATER_CUTOFF: f32 = 0.3;
pub const TERRAIN_SAND_CUTOFF: f32 = TERRAIN_WATER_CUTOFF + TERRAIN_SAND_EPSILON;
pub const TERRAIN_CHUNK_VARIATION_SCALE: f32 = 15.0;
pub const TERRAIN_BIOME_VARIATION_SCALE: f32 = 15.0;
pub const TERRAIN_BIOME_MEADOW_CUTOFF: f32 = 0.8;

pub mod actors {
    pub const TREE_TOTAL_RADIUS: f32 = 1.0;
    pub const TREE_TOTAL_HEIGHT: f32 = 2.0;
    pub const HUMAN_TOTAL_RADIUS: f32 = 0.8;
    pub const HUMAN_TOTAL_HEIGHT: f32 = 1.8;
}

pub const TERRAIN_MESH_THICKNESS: f32 = 1.0;
const TERRAIN_EPSILON: f32 = 0.1;
pub const TERRAIN_MESH_WIDTH: f32 = TERRAIN_CHUNK_SIZE - TERRAIN_EPSILON;

pub mod materials {
    pub mod actor {
        use bevy::prelude::*;
        pub const TREE_TRUNK_COLOR: Color = Color::srgb(0.54, 0.27, 0.08);
        pub const TREE_TOP_COLOR: Color = Color::srgb(0.38, 0.54, 0.24);
        pub const HUMAN_COLOR: Color = Color::WHITE;
    }

    pub mod terrain {
        use bevy::prelude::*;
        pub const MEADOW_GRASS_COLOR: Color = Color::srgb(0.3, 0.8, 0.3);
        pub const MEADOW_WATER_COLOR: Color = Color::srgb(0.1, 0.1, 0.9);
        pub const MEADOW_SAND_COLOR: Color = Color::srgb(0.99, 0.93, 0.44);
        pub const SNOW_GRASS_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
        pub const SNOW_SAND_COLOR: Color = Color::srgb(210.0 / 255.0, 170.0 / 255.0, 109.0 / 255.0);
        pub const SNOW_WATER_COLOR: Color = Color::srgb(0.7, 0.7, 0.9);
    }
}

pub mod camera {
    //! Constants associated with the camera.
    use bevy::prelude::*;
    pub const DEFAULT_SPEED: f32 = 5.0;
    pub const DEFAULT_SPRINT_MODIFIER: f32 = 4.0; // Factor by which we increase speed.
    pub const SENSITIVITY: f32 = 0.5; // How quickly we yaw and pitch.
    pub const DEFAULT_PITCH: f32 = 0.0;
    pub const DEFAULT_YAW: f32 = 0.0;
    pub const DEFAULT_POSITION: Vec3 = Vec3::ZERO;
}

pub mod meshes {
    use crate::well_known_terms::actors::*;

    pub const HUMAN_RADIUS: f32 = HUMAN_TOTAL_RADIUS;
    pub const HUMAN_HEIGHT: f32 = HUMAN_TOTAL_HEIGHT;
    pub const TREE_TRUNK_RADIUS: f32 = 0.4;
    pub const TREE_TRUNK_HEIGHT: f32 = TREE_TOTAL_HEIGHT - TREE_TOTAL_RADIUS;
    pub const TREE_TOP_RADIUS: f32 = TREE_TOTAL_RADIUS;
}
