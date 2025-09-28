//! # Graphics
//! This module attached graphics to entities that are primarily controlled by yv_core.
//!
//! The graphics system optimizes asset usage by caching mesh and material handles
//! rather than creating new assets for each terrain chunk.
//! This reduces memory usage and improves rendering performance through Bevy's automatic
//! instancing.

pub mod components;
pub mod plugin;
pub mod systems;

pub use plugin::GraphicsPlugin;
