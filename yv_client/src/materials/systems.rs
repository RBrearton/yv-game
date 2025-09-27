use bevy::prelude::*;

use crate::materials::resources::{ActorMaterials, TerrainMaterials};
use crate::well_known_terms::materials;

/// # Initialize terrain materials
/// This system initializes all the terrain materials.
/// This makes it so that initialization happens exactly once, and we always have access to a handle
/// to each terrain material.
pub(super) fn init_terrain_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut terrain_materials: ResMut<TerrainMaterials>,
) {
    let meadow_grass_material = materials.add(StandardMaterial {
        base_color: materials::terrain::MEADOW_GRASS_COLOR,
        ..default()
    });
    let meadow_water_material = materials.add(StandardMaterial {
        base_color: materials::terrain::MEADOW_WATER_COLOR,
        ..default()
    });
    let meadow_sand_material = materials.add(StandardMaterial {
        base_color: materials::terrain::MEADOW_SAND_COLOR,
        ..default()
    });
    let snow_grass_material = materials.add(StandardMaterial {
        base_color: materials::terrain::SNOW_GRASS_COLOR,
        ..default()
    });
    let snow_water_material = materials.add(StandardMaterial {
        base_color: materials::terrain::SNOW_WATER_COLOR,
        ..default()
    });
    let snow_sand_material = materials.add(StandardMaterial {
        base_color: materials::terrain::SNOW_SAND_COLOR,
        ..default()
    });
    terrain_materials.meadow_grass = meadow_grass_material;
    terrain_materials.meadow_water = meadow_water_material;
    terrain_materials.meadow_sand = meadow_sand_material;
    terrain_materials.snow_grass = snow_grass_material;
    terrain_materials.snow_water = snow_water_material;
    terrain_materials.snow_sand = snow_sand_material;
}

/// # Initialize actor materials
/// This system initializes all the actor materials.
/// This makes it so that initialization happens exactly once, and we always have access to a handle
/// to each actor material.
pub(super) fn init_actor_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut actor_materials: ResMut<ActorMaterials>,
) {
    let tree_trunk_material = materials.add(StandardMaterial {
        base_color: materials::actor::TREE_TRUNK_COLOR,
        ..default()
    });
    let tree_top_material = materials.add(StandardMaterial {
        base_color: materials::actor::TREE_TOP_COLOR,
        ..default()
    });
    let human_material = materials.add(StandardMaterial {
        base_color: materials::actor::HUMAN_COLOR,
        ..default()
    });
    actor_materials.tree_trunk = tree_trunk_material;
    actor_materials.tree_top = tree_top_material;
    actor_materials.human = human_material;
}
