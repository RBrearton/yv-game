use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use yv_core::YvCorePlugins;

use camera::CameraPlugin;
use scenes::ScenesPlugin;
use ui::UIPlugin;

mod camera;
mod graphics;
mod scenes;
mod ui;
mod well_known_terms;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(YvCorePlugins)
        .add_plugins(ScenesPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
