use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use yv_core::YvCorePlugins;

use camera::CameraPlugin;
use graphics::GraphicsPlugin;
use scenes::ScenesPlugin;
use ui::UIPlugin;

mod camera;
mod graphics;
mod inputs;
mod lighting;
mod materials;
mod meshes;
mod player;
mod scenes;
mod ui;
mod well_known_terms;

use crate::{
    inputs::InputsPlugin, lighting::plugin::LightingPlugin, materials::MaterialsPlugin,
    meshes::MeshesPlugin, player::plugin::PlayerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(MeshPickingPlugin)
        .add_plugins(YvCorePlugins)
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
