use bevy::prelude::*;

use crate::meshes::{resources::Meshes, systems::init_meshes, MeshesInitSystems};

/// # Geometry Plugin
/// This plugin is responsible for handling/loading whatever custom geometry we need to run the
/// game.
pub struct MeshesPlugin;

impl Plugin for MeshesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_meshes.in_set(MeshesInitSystems));
        app.insert_resource(Meshes::default());
    }
}
