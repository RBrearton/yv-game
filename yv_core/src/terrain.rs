use bevy::prelude::*;
use hashbrown::HashMap;

use crate::well_known_terms::TERRAIN_CHUNK_SIZE;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainChunks>();
        app.add_event::<AddTerrainChunk>();
        app.add_event::<RemoveTerrainChunk>();
        app.add_systems(
            Update,
            add_terrain_chunks.run_if(on_event::<AddTerrainChunk>),
        );
        app.add_systems(
            Update,
            remove_terrain_chunks.run_if(on_event::<RemoveTerrainChunk>),
        );
    }
}

fn add_terrain_chunks(
    mut commands: Commands,
    mut terrain_chunks: ResMut<TerrainChunks>,
    mut chunk_events: EventReader<AddTerrainChunk>,
    mut spawned_events: EventWriter<TerrainChunkSpawned>,
) {
    for event in chunk_events.read() {
        // Spawn the chunk, and add its ID to the TerrainChunks resource.
        let new_chunk_id = commands.spawn(event.get_chunk()).id();
        terrain_chunks.chunks.insert(event.index, new_chunk_id);
        spawned_events.write(TerrainChunkSpawned {
            data: event.get_chunk(),
            entity: new_chunk_id,
        });
    }
}

fn remove_terrain_chunks(
    mut commands: Commands,
    mut terrain_chunks: ResMut<TerrainChunks>,
    mut chunk_events: EventReader<RemoveTerrainChunk>,
) {
    for event in chunk_events.read() {
        if let Some(entity_id) = terrain_chunks.chunks.remove(&event.index) {
            commands.entity(entity_id).despawn();
        }
    }
}

/// The resource that contains references to all terrain chunks.
#[derive(Resource, Default)]
struct TerrainChunks {
    chunks: HashMap<TerrainIndex, Entity>,
}

/// The event that, when fired, adds a terrain chunk to the world.
#[derive(Event, Clone, Debug)]
pub struct AddTerrainChunk {
    index: TerrainIndex,
    chunk_type: ChunkType,
    biome: Biome,
}
impl AddTerrainChunk {
    fn get_chunk(&self) -> TerrainChunk {
        TerrainChunk {
            index: self.index,
            chunk_type: self.chunk_type,
            biome: self.biome,
        }
    }
    pub fn new(index: TerrainIndex, chunk_type: ChunkType, biome: Biome) -> Self {
        Self {
            index,
            chunk_type,
            biome,
        }
    }
}

/// The event that, when fired, removes a terrain chunk from the world.
#[derive(Event, Debug, Clone)]
pub struct RemoveTerrainChunk {
    index: TerrainIndex,
}

/// Event fired whenever a terrain chunk is spawned.
#[derive(Event, Debug, Clone)]
pub struct TerrainChunkSpawned {
    pub data: TerrainChunk,
    pub entity: Entity,
}

/// A 2D index that uniquely specifies a chunk of terrain.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct TerrainIndex {
    x: i32,
    y: i32,
}
impl TerrainIndex {
    /// Get the terrain index from a 2D position.
    pub fn from_position(x: f32, y: f32) -> Self {
        Self {
            x: (x / TERRAIN_CHUNK_SIZE).floor() as i32,
            y: (y / TERRAIN_CHUNK_SIZE).floor() as i32,
        }
    }
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// The type of terrain in a chunk.
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum ChunkType {
    Grass,
    Water,
}

/// The biome of a chunk.
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum Biome {
    Meadow,
    Snow,
}

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct TerrainChunk {
    index: TerrainIndex,
    chunk_type: ChunkType,
    biome: Biome,
}
