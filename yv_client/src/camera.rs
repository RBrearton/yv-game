use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetCamera>();
        app.add_systems(Startup, setup_camera_system);
        app.add_systems(Update, update_camera_system.in_set(UpdateCameraSystems));
    }
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

/// Fire this event to directly update the camera.
#[derive(Event)]
pub struct SetCamera {
    pub translation: Vec3,
    pub up: Vec3,
    pub target: Vec3,
}

/// All camera update systems run in the update camera systems set.
/// This should run after the core game logic updates.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateCameraSystems;

fn update_camera_system(
    mut camera: Query<&mut Transform, With<Camera3d>>,
    mut update_camera: EventReader<SetCamera>,
) {
    let mut camera = match camera.single_mut() {
        Ok(camera) => camera,
        Err(_) => {
            warn!("Tried to update the camera, but no camera was found.");
            return;
        }
    };

    // Iterate over all events.
    // We permit multiple events to be fired in a single frame and just take the last one.
    for event in update_camera.read() {
        camera.look_at(event.target, event.up);
        camera.translation = event.translation;
    }
}
