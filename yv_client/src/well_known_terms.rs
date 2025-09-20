//! # Well known terms
//! This module contains some constants and terms that are used throughout the project.

use bevy::prelude::*;

pub const TERRAIN_THICKNESS: f32 = 1.0;
pub const TERRAIN_MATERIAL_COLOR_MEADOW_GRASS: Color = Color::srgb(0.3, 0.8, 0.3);
pub const TERRAIN_MATERIAL_COLOR_MEADOW_WATER: Color = Color::srgb(0.1, 0.1, 0.9);
pub const TERRAIN_MATERIAL_COLOR_SNOW_GRASS: Color = Color::srgb(0.9, 0.9, 0.9);
pub const TERRAIN_MATERIAL_COLOR_SNOW_WATER: Color = Color::srgb(0.7, 0.7, 0.9);

pub mod camera {
    //! Constants associated with the camera.
    use bevy::prelude::*;
    pub const DEFAULT_SPEED: f32 = 1.0;
    pub const DEFAULT_SPRINT_MODIFIER: f32 = 2.0; // Factor by which we increase speed.
    pub const SENSITIVITY: f32 = 0.001; // How quickly we yaw and pitch.
    pub const DEFAULT_PITCH: f32 = 0.0;
    pub const DEFAULT_YAW: f32 = 0.0;
    pub const DEFAULT_POSITION: Vec3 = Vec3::ZERO;
}
