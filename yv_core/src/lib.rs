use bevy::prelude::*;

pub mod actor;
pub mod noise;
pub mod terrain;
pub mod well_known_terms;

pub struct YvCorePlugins;

impl Plugin for YvCorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(terrain::TerrainPlugin)
            .add_plugins(actor::plugin::ActorPlugin);
    }
}
