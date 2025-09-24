use bevy::prelude::*;
use hashbrown::HashMap;

use crate::noise::perlin_noise;

use crate::well_known_terms::{
    TERRAIN_BIOME_MEADOW_CUTOFF, TERRAIN_BIOME_VARIATION_SCALE, TERRAIN_CHUNK_SIZE,
    TERRAIN_CHUNK_VARIATION_SCALE, TERRAIN_CONFIG_SEED, TERRAIN_SAND_CUTOFF, TERRAIN_WATER_CUTOFF,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainChunks>();
        app.init_resource::<TerrainGenConfig>();
        app.add_event::<AddTerrainChunk>();
        app.add_event::<RemoveTerrainChunk>();
        app.add_event::<ProcedurallyGenerateTerrain>();
        app.add_event::<TerrainChunkSpawned>();
        app.add_systems(
            Update,
            (
                remove_terrain_chunks.run_if(on_event::<RemoveTerrainChunk>),
                procedurally_generate_terrain.run_if(on_event::<ProcedurallyGenerateTerrain>),
                add_terrain_chunks.run_if(on_event::<AddTerrainChunk>),
            )
                .chain()
                .in_set(UpdateTerrainSystems),
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
        let new_chunk_id = commands
            .spawn((
                event.get_chunk(),
                event.get_chunk().index.to_transform(),
                InheritedVisibility::VISIBLE,
            ))
            .id();
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

fn procedurally_generate_terrain(
    mut evr_chunk_events: EventReader<ProcedurallyGenerateTerrain>,
    mut evw_add_terrain_chunk: EventWriter<AddTerrainChunk>,
    terrain_gen_config: Res<TerrainGenConfig>,
) {
    for event in evr_chunk_events.read() {
        for index in event.terrain_indices.clone() {
            // Procedurally generate a new chunk.
            let new_chunk = TerrainChunk::new_procedural(index, &terrain_gen_config);

            // We don't actually add the chunk here - we hand over to the add_terrain_chunks system.
            evw_add_terrain_chunk.write(AddTerrainChunk::new(
                index,
                new_chunk.chunk_type,
                new_chunk.biome,
            ));
        }
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateTerrainSystems;

/// The resource that contains references to all terrain chunks.
#[derive(Resource, Default)]
struct TerrainChunks {
    chunks: HashMap<TerrainIndex, Entity>,
}

/// The resource that stores config used to generate terrain.
#[derive(Resource)]
pub struct TerrainGenConfig {
    seed: u32,

    water_cutoff: f32,
    sand_cutoff: f32,
    chunk_variation_scale: f32,

    meadow_cutoff: f32,
    biome_variation_scale: f32,
}
impl Default for TerrainGenConfig {
    fn default() -> Self {
        Self {
            seed: TERRAIN_CONFIG_SEED,
            water_cutoff: TERRAIN_WATER_CUTOFF,
            sand_cutoff: TERRAIN_SAND_CUTOFF,
            chunk_variation_scale: TERRAIN_CHUNK_VARIATION_SCALE,
            meadow_cutoff: TERRAIN_BIOME_MEADOW_CUTOFF,
            biome_variation_scale: TERRAIN_BIOME_VARIATION_SCALE,
        }
    }
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
impl TerrainChunkSpawned {
    pub fn chunk_type(&self) -> ChunkType {
        self.data.chunk_type
    }
    pub fn biome(&self) -> Biome {
        self.data.biome
    }
    pub fn index(&self) -> TerrainIndex {
        self.data.index
    }
    pub fn transform(&self) -> Transform {
        self.data.index.to_transform()
    }
}

/// Event fired to generate terrain.
/// We can always generate terrain on the fly, but it is often useful to batch generate.
#[derive(Event, Debug, Clone)]
pub struct ProcedurallyGenerateTerrain {
    terrain_indices: Vec<TerrainIndex>,
}
impl ProcedurallyGenerateTerrain {
    /// Generate terrain for some specific indices.
    pub fn new(indices: Vec<TerrainIndex>) -> Self {
        Self {
            terrain_indices: indices,
        }
    }
    /// Generate terrain for a range of indices.
    pub fn new_range(x_idx_start: i32, x_idx_end: i32, y_idx_start: i32, y_idx_end: i32) -> Self {
        Self {
            terrain_indices: (x_idx_start..x_idx_end)
                .flat_map(|x| (y_idx_start..y_idx_end).map(move |y| TerrainIndex::new(x, y)))
                .collect(),
        }
    }
}

/// A 2D index that uniquely specifies a chunk of terrain.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct TerrainIndex {
    pub x: i32,
    pub y: i32,
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
    pub fn to_position(self) -> Vec3 {
        Vec3::new(
            self.x as f32 * TERRAIN_CHUNK_SIZE,
            self.y as f32 * TERRAIN_CHUNK_SIZE,
            0.0,
        )
    }
    pub fn to_transform(self) -> Transform {
        Transform::from_translation(self.to_position())
    }
}

/// The type of terrain in a chunk.
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash, Default)]
pub enum ChunkType {
    /// The most common thing we see in the terrain.
    #[default]
    Grass,

    /// Sand is the chunk type that tends to surround water.
    Sand,

    /// There are localized bodies of water; you can't path through water.
    /// In a volcano biome this would be lava, in snowy biomes this would be ice etc.
    Water,
}
impl ChunkType {
    pub fn from_random_number(rn: f32, config: &TerrainGenConfig) -> Self {
        if rn < config.water_cutoff {
            Self::Water
        } else if rn < config.sand_cutoff {
            Self::Sand
        } else {
            Self::Grass
        }
    }
}

/// The biome of a chunk.
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash, Default)]
pub enum Biome {
    #[default]
    Meadow,
    Snow,
}
impl Biome {
    pub fn from_random_number(rn: f32, config: &TerrainGenConfig) -> Self {
        if rn < config.meadow_cutoff {
            Self::Meadow
        } else {
            Self::Snow
        }
    }
}

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct TerrainChunk {
    pub index: TerrainIndex,
    pub chunk_type: ChunkType,
    pub biome: Biome,
}
impl TerrainChunk {
    // Procedurally generate a terrain chunk.
    pub fn new_procedural(index: TerrainIndex, config: &TerrainGenConfig) -> Self {
        let x = index.x as f32;
        let y = index.y as f32;
        let chunk_noise = perlin_noise(x, y, config.chunk_variation_scale, config.seed);
        let biome_noise = perlin_noise(x, y, config.biome_variation_scale, config.seed);
        let chunk_type = ChunkType::from_random_number(chunk_noise, config);
        let biome = Biome::from_random_number(biome_noise, config);
        Self {
            index,
            chunk_type,
            biome,
        }
    }
}
