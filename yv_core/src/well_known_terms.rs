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
