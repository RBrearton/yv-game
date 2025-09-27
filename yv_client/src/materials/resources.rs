use bevy::prelude::*;

/// # Materials
/// This resource holds handles to all custom materials that we want to use in the game.
#[derive(Resource, Reflect, Default)]
pub struct TerrainMaterials {
    /// The handle to the meadow grass material.
    pub meadow_grass: Handle<StandardMaterial>,

    /// The handle to the meadow water material.
    pub meadow_water: Handle<StandardMaterial>,

    /// The handle to the meadow sand material.
    pub meadow_sand: Handle<StandardMaterial>,

    /// The handle to the snow grass material.
    pub snow_grass: Handle<StandardMaterial>,

    /// The handle to the snow water material.
    pub snow_water: Handle<StandardMaterial>,

    /// The handle to the snow sand material.
    pub snow_sand: Handle<StandardMaterial>,
}

/// # Actor materials
/// This resource holds handles to all custom materials that we need to render actors.
#[derive(Resource, Reflect, Default)]
pub struct ActorMaterials {
    /// The handle to the tree trunk material.
    pub tree_trunk: Handle<StandardMaterial>,

    /// The handle to the tree top material.
    pub tree_top: Handle<StandardMaterial>,

    /// The handle to the human material.
    pub human: Handle<StandardMaterial>,
}
