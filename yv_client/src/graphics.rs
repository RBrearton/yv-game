//! # Graphics
//! This module attached graphics to entities that are primarily controlled by yv_core.
//!
//! The graphics system optimizes asset usage by caching mesh and material handles
//! rather than creating new assets for each terrain chunk.
//! This reduces memory usage and improves rendering performance through Bevy's automatic
//! instancing.

use bevy::prelude::*;
use yv_core::{
    actor::{ActorSpawned, ActorType},
    terrain::{Biome, ChunkType, TerrainChunkSpawned},
};

use crate::meshes::Meshes;
use crate::{materials::resources::*, well_known_terms::TERRAIN_MESH_THICKNESS};

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                render_terrain.run_if(on_event::<TerrainChunkSpawned>),
                render_actors.run_if(on_event::<ActorSpawned>),
            ),
        );
    }
}

fn render_actors(
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
                entity.with_children(|parent| {
                    parent.spawn((
                        Mesh3d(actor_meshes.player.clone()),
                        MeshMaterial3d(actor_materials.human.clone()),
                    ));
                });
            }
            ActorType::Tree => {
                // Create a child entity of the tree entity.
                entity.with_children(|parent| {
                    parent.spawn((
                        Mesh3d(actor_meshes.tree_trunk.clone()),
                        MeshMaterial3d(actor_materials.tree_trunk.clone()),
                    ));
                });
            }
        }
    }
}

fn render_terrain(
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
                Mesh3d(mesh),             // The mesh to render for this terrain chunk.
                MeshMaterial3d(material), // The material to use for rendering.
                child_transform,          // The transform for positioning the mesh.
            ));
        });
    }
}
