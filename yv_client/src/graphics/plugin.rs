use bevy::prelude::*;
use yv_core::{actor::ActorSpawned, terrain::TerrainChunkSpawned};

use super::systems::*;

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
