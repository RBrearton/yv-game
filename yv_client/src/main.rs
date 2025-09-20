use bevy::prelude::*;

use camera::CameraPlugin;
use scenes::ScenesPlugin;
use ui::UIPlugin;

mod camera;
mod scenes;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenesPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
