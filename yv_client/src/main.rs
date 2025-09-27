use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use yv_core::YvCorePlugins;

use camera::CameraPlugin;
use graphics::GraphicsPlugin;
use scenes::ScenesPlugin;
use ui::UIPlugin;

use crate::{inputs::InputsPlugin, meshes::MeshesPlugin};

mod camera;
mod graphics;
mod inputs;
mod meshes;
mod scenes;
mod ui;
mod utils;
mod well_known_terms;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(YvCorePlugins)
        .add_plugins(MeshesPlugin)
        .add_plugins(GraphicsPlugin)
        .add_plugins(ScenesPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(InputsPlugin)
        .run();
}
