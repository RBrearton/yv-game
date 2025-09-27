use bevy::prelude::*;

use crate::materials::{
    resources::{ActorMaterials, TerrainMaterials},
    system_sets::MaterialsInitSystems,
    systems::{init_actor_materials, init_terrain_materials},
};

/// # Materials plugin
/// The plugin responsible for handling/loading all custom materials used in the game.
pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainMaterials::default());
        app.insert_resource(ActorMaterials::default());
        app.add_systems(
            Startup,
            (init_terrain_materials, init_actor_materials).in_set(MaterialsInitSystems),
        );
    }
}
