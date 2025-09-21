//! # Graphics
//! This module attached graphics to entities that are primarily controlled by yv_core.
//!
//! The graphics system optimizes asset usage by caching mesh and material handles
//! rather than creating new assets for each terrain chunk.
//! This reduces memory usage and improves rendering performance through Bevy's automatic
//! instancing.

use bevy::prelude::*;
use hashbrown::HashMap;
use yv_core::terrain::{Biome, ChunkType, TerrainChunkSpawned};

use crate::well_known_terms::{
    TERRAIN_MATERIAL_COLOR_MEADOW_GRASS, TERRAIN_MATERIAL_COLOR_MEADOW_SAND,
    TERRAIN_MATERIAL_COLOR_MEADOW_WATER, TERRAIN_MATERIAL_COLOR_SNOW_GRASS,
    TERRAIN_MATERIAL_COLOR_SNOW_SAND, TERRAIN_MATERIAL_COLOR_SNOW_WATER, TERRAIN_MESH_THICKNESS,
    TERRAIN_MESH_WIDTH,
};

/// Resource that caches mesh and material handles to avoid creating duplicate assets.
/// This enables Bevy's automatic instancing for better performance.
#[derive(Resource, Default)]
struct TerrainAssets {
    /// Cached mesh handles for different terrain chunk types.
    meshes: HashMap<ChunkType, Handle<Mesh>>,
    /// Cached material handles for different biome-chunk combinations.
    materials: HashMap<(Biome, ChunkType), Handle<StandardMaterial>>,
}

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainAssets>();
        app.add_systems(Update, attach_terrain_meshes);
    }
}

fn attach_terrain_meshes(
    mut commands: Commands,
    mut spawned_events: EventReader<TerrainChunkSpawned>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut terrain_assets: ResMut<TerrainAssets>,
) {
    for event in spawned_events.read() {
        // Get or create the appropriate mesh and material based on chunk type and biome.
        let mesh = get_or_create_terrain_mesh(event.chunk_type(), &mut meshes, &mut terrain_assets);
        let material = get_or_create_terrain_material(
            event.chunk_type(),
            event.biome(),
            &mut materials,
            &mut terrain_assets,
        );

        // Attach rendering components to the existing entity.
        commands.entity(event.entity).insert((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            event.transform(),
        ));
    }
}

/// Gets or creates a cached mesh handle for the given chunk type.
/// This ensures that identical terrain chunks share the same mesh asset.
fn get_or_create_terrain_mesh(
    chunk_type: ChunkType,
    meshes: &mut ResMut<Assets<Mesh>>,
    terrain_assets: &mut ResMut<TerrainAssets>,
) -> Handle<Mesh> {
    if let Some(handle) = terrain_assets.meshes.get(&chunk_type) {
        return handle.clone();
    }

    // Create mesh based on chunk type.
    let mesh = match chunk_type {
        ChunkType::Grass => meshes.add(Cuboid::new(
            TERRAIN_MESH_WIDTH,
            TERRAIN_MESH_WIDTH,
            TERRAIN_MESH_THICKNESS,
        )),
        ChunkType::Sand => meshes.add(Cuboid::new(
            TERRAIN_MESH_WIDTH,
            TERRAIN_MESH_WIDTH,
            TERRAIN_MESH_THICKNESS,
        )),
        ChunkType::Water => meshes.add(Cuboid::new(
            TERRAIN_MESH_WIDTH,
            TERRAIN_MESH_WIDTH,
            TERRAIN_MESH_THICKNESS,
        )),
    };

    // Cache the handle for future use.
    terrain_assets.meshes.insert(chunk_type, mesh.clone());
    mesh
}

/// Gets or creates a cached material handle for the given biome and chunk type combination.
/// This ensures that terrain chunks with identical appearance share the same material asset.
fn get_or_create_terrain_material(
    chunk_type: ChunkType,
    biome: Biome,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    terrain_assets: &mut ResMut<TerrainAssets>,
) -> Handle<StandardMaterial> {
    let key = (biome, chunk_type);
    if let Some(handle) = terrain_assets.materials.get(&key) {
        return handle.clone();
    }

    // Create material based on biome and chunk type.
    let material = match biome {
        Biome::Meadow => match chunk_type {
            ChunkType::Grass => materials.add(StandardMaterial {
                base_color: TERRAIN_MATERIAL_COLOR_MEADOW_GRASS,
                ..default()
            }),
            ChunkType::Sand => materials.add(StandardMaterial {
                base_color: TERRAIN_MATERIAL_COLOR_MEADOW_SAND,
                ..default()
            }),
            ChunkType::Water => materials.add(StandardMaterial {
                base_color: TERRAIN_MATERIAL_COLOR_MEADOW_WATER,
                ..default()
            }),
        },
        Biome::Snow => match chunk_type {
            ChunkType::Grass => materials.add(StandardMaterial {
                base_color: TERRAIN_MATERIAL_COLOR_SNOW_GRASS,
                ..default()
            }),
            ChunkType::Sand => materials.add(StandardMaterial {
                base_color: TERRAIN_MATERIAL_COLOR_SNOW_SAND,
                ..default()
            }),
            ChunkType::Water => materials.add(StandardMaterial {
                base_color: TERRAIN_MATERIAL_COLOR_SNOW_WATER,
                ..default()
            }),
        },
    };

    // Cache the handle for future use.
    terrain_assets.materials.insert(key, material.clone());
    material
}
