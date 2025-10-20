use std::f32::consts::PI;

use crate::{
    actor::{ActorSpawned, ActorType},
    terrain::{Biome, ChunkType, TerrainChunkSpawned},
};
use bevy::prelude::*;

use crate::{
    graphics::components::HighlightOnHover,
    materials::resources::{ActorMaterials, TerrainMaterials},
    meshes::Meshes,
    well_known_terms::*,
};

// pub fn render_physical(
//     mut commands: Commands,
//     mut evr_spawned: EventReader<RenderableSpawned>,
//     meshes: Res<Meshes>,
//     materials: Res<AllMaterials>,
// ) {
//     for event in evr_spawned.read() {
//         // ...
//     }
// }

/// The system ultimately responsible for attaching meshes and materials to actors.
pub fn render_actors(
    mut commands: Commands,
    mut evr_spawned: EventReader<ActorSpawned>,
    actor_meshes: Res<Meshes>,
    actor_materials: Res<ActorMaterials>,
) {
    for event in evr_spawned.read() {
        let mut entity = commands.entity(event.entity);
        match event.actor_type {
            ActorType::Player => {
                // Create a child entity of the player entity.
                let mut child_transform = Transform::from_translation(Vec3::ZERO);
                let human_mesh_height = meshes::HUMAN_HEIGHT + 2.0 * meshes::HUMAN_RADIUS;
                child_transform.translation.z += human_mesh_height / 2.0;
                child_transform.rotate_x(-PI / 2.0); // Because mesh goes along y by default.
                entity.with_children(|parent| {
                    parent.spawn((
                        Mesh3d(actor_meshes.player.clone()),
                        MeshMaterial3d(actor_materials.human.clone()),
                        child_transform,
                    ));
                });
            }
            ActorType::Tree => {
                // Create a child entity of the tree entity.
                let trunk_height = meshes::TREE_TRUNK_HEIGHT;
                let top_section_height = meshes::TREE_TOP_RADIUS * 2.0;
                let mut trunk_transform = Transform::from_translation(Vec3::ZERO);
                trunk_transform.translation.z += trunk_height / 2.0;
                trunk_transform.rotate_x(-PI / 2.0);
                let mut top_transform = Transform::from_translation(Vec3::ZERO);
                top_transform.translation.z += trunk_height + top_section_height / 2.0;
                entity.with_children(|parent| {
                    parent.spawn((
                        Mesh3d(actor_meshes.tree_trunk.clone()),
                        MeshMaterial3d(actor_materials.tree_trunk.clone()),
                        trunk_transform,
                    ));
                    parent.spawn((
                        Mesh3d(actor_meshes.tree_top.clone()),
                        MeshMaterial3d(actor_materials.tree_top.clone()),
                        top_transform,
                    ));
                });
            }
        }
    }
}

/// The system ultimately responsible for attaching meshes and materials to terrain chunks.
pub fn render_terrain(
    mut commands: Commands,
    mut spawned_events: EventReader<TerrainChunkSpawned>,
    meshes: Res<Meshes>,
    materials: Res<TerrainMaterials>,
) {
    for event in spawned_events.read() {
        // Get or create the appropriate mesh and material based on chunk type and biome.
        let mesh = meshes.terrain.clone();
        let material = match event.biome() {
            Biome::Meadow => match event.chunk_type() {
                ChunkType::Grass => materials.meadow_grass.clone(),
                ChunkType::Sand => materials.meadow_sand.clone(),
                ChunkType::Water => materials.meadow_water.clone(),
            },
            Biome::Snow => match event.chunk_type() {
                ChunkType::Grass => materials.snow_grass.clone(),
                ChunkType::Sand => materials.snow_sand.clone(),
                ChunkType::Water => materials.snow_water.clone(),
            },
        };

        // The mesh is centered at the origin, so we need to offset it down by half the thickness.
        let mut child_transform = Transform::from_translation(Vec3::ZERO);
        child_transform.translation.z -= TERRAIN_MESH_THICKNESS / 2.0;
        commands.entity(event.entity).with_children(|parent| {
            parent.spawn((
                HighlightOnHover,
                Mesh3d(mesh),             // The mesh to render for this terrain chunk.
                MeshMaterial3d(material), // The material to use for rendering.
                child_transform,          // The transform for positioning the mesh.
            ));
        });
    }
}
