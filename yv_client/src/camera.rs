use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera_system);
    }
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
