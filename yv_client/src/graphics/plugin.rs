use bevy::prelude::*;
use yv_core::{actor::ActorSpawned, terrain::TerrainChunkSpawned};

use super::{observers::*, resources::*, systems::*};

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoverHighlightConfig>();

        app.add_observer(highlight_on_hover);
        app.add_observer(unhighlight_on_out);

        app.add_systems(
            Update,
            (
                render_terrain.run_if(on_event::<TerrainChunkSpawned>),
                render_actors.run_if(on_event::<ActorSpawned>),
            ),
        );
    }
}
