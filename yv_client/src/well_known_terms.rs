//! # Well known terms
//! This module contains some constants and terms that are used throughout the project.

use bevy::prelude::*;
use yv_core::well_known_terms::TERRAIN_CHUNK_SIZE;

pub const TERRAIN_MESH_THICKNESS: f32 = 1.0;
const TERRAIN_EPSILON: f32 = 0.1;
pub const TERRAIN_MESH_WIDTH: f32 = TERRAIN_CHUNK_SIZE - TERRAIN_EPSILON;
pub const TERRAIN_MATERIAL_COLOR_MEADOW_GRASS: Color = Color::srgb(0.3, 0.8, 0.3);
pub const TERRAIN_MATERIAL_COLOR_MEADOW_WATER: Color = Color::srgb(0.1, 0.1, 0.9);
pub const TERRAIN_MATERIAL_COLOR_MEADOW_SAND: Color = Color::srgb(0.99, 0.93, 0.44);
pub const TERRAIN_MATERIAL_COLOR_SNOW_GRASS: Color = Color::srgb(0.9, 0.9, 0.9);
pub const TERRAIN_MATERIAL_COLOR_SNOW_SAND: Color =
    Color::srgb(210.0 / 255.0, 170.0 / 255.0, 109.0 / 255.0);
pub const TERRAIN_MATERIAL_COLOR_SNOW_WATER: Color = Color::srgb(0.7, 0.7, 0.9);

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
