use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use camera::CameraPlugin;
use graphics::GraphicsPlugin;
use scenes::ScenesPlugin;
use ui::UIPlugin;

mod actor;
mod camera;
mod geometry;
mod graphics;
mod inputs;
mod lighting;
mod materials;
mod meshes;
mod noise;
mod player;
mod scenes;
mod terrain;
mod ui;
mod well_known_terms;

use crate::{
    actor::plugin::ActorPlugin, inputs::InputsPlugin, lighting::plugin::LightingPlugin,
    materials::MaterialsPlugin, meshes::MeshesPlugin, player::plugin::PlayerPlugin,
    terrain::TerrainPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(MeshPickingPlugin)
        .add_plugins(ActorPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(LightingPlugin)
        .add_plugins(MaterialsPlugin)
        .add_plugins(MeshesPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GraphicsPlugin)
        .add_plugins(ScenesPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(InputsPlugin)
        .insert_resource(MeshPickingSettings {
            require_markers: true, // only pick things explicitly marked
            ..default()
        })
        .run();
}
