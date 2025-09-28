use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    input::mouse::MouseMotion,
    prelude::*,
};
use std::f32::consts;

use crate::{inputs::InputMap, well_known_terms::camera};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetCamera>();
        app.init_state::<CameraMode>();
        app.init_resource::<FreeCameraState>();
        app.add_systems(Startup, setup_camera_system);
        app.add_systems(
            Update,
            (
                free_camera_update.run_if(in_state(CameraMode::Free)),
                update_camera_system,
            )
                .chain()
                .in_set(UpdateCameraSystems),
        );
    }
}

/// This system sets up the camera.
fn setup_camera_system(mut commands: Commands) {
    commands.spawn((
        MeshPickingCamera,
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
    ));
}

/// Fire this event to directly update the camera.
#[derive(Event)]
pub struct SetCamera {
    pub translation: Vec3,
    pub up: Vec3,
    pub target: Vec3,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraMode {
    #[default]
    Free,
    FollowPlayer,
}
impl std::fmt::Display for CameraMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Resource)]
pub struct FreeCameraState {
    pub pitch: f32,
    pub yaw: f32,
    pub base_speed: f32,
    pub is_sprinting: bool,
    pub sprint_modifier: f32,
    pub sensitivity: f32,
    pub translation: Vec3,
}
impl Default for FreeCameraState {
    fn default() -> Self {
        Self {
            pitch: camera::DEFAULT_PITCH,
            yaw: camera::DEFAULT_YAW,
            base_speed: camera::DEFAULT_SPEED,
            is_sprinting: false,
            sprint_modifier: camera::DEFAULT_SPRINT_MODIFIER,
            sensitivity: camera::SENSITIVITY,
            translation: camera::DEFAULT_POSITION,
        }
    }
}
impl FreeCameraState {
    /// Returns the normal parallel to the camera's direction.
    pub fn forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
        )
    }

    /// Returns the speed of the camera.
    /// Of course, this isn't actually speed - it's the amount of distance that we want to move
    /// this frame - but the code is easier to understand if we call this "speed".
    pub fn speed(&self, delta_time: f32) -> f32 {
        if self.is_sprinting {
            self.base_speed * self.sprint_modifier * delta_time
        } else {
            self.base_speed * delta_time
        }
    }

    /// Return a SetCamera event that requests that we update the camera's transform to match the
    /// current state of the free camera.
    pub fn set_camera(&self) -> SetCamera {
        SetCamera {
            translation: self.translation,
            up: Vec3::Z,
            target: self.translation + self.forward(),
        }
    }
}

/// All camera update systems run in the update camera systems set.
/// This should run after the core game logic updates.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateCameraSystems;

/// This is the fundamental system that actually updates the camera.
/// To update the camera, you should always fire a SetCamera event.
/// Any other camera mutation systems should eventually end up calling this system.
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
        camera.translation = event.translation;
        camera.look_at(event.target, event.up);
    }
}

/// This system updates the camera while we're in free camera mode.
/// Free camera modes are characterized by cameras that pitch and yaw, but never roll.
fn free_camera_update(
    mut free_camera_state: ResMut<FreeCameraState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut evr_mouse_motion: EventReader<MouseMotion>,
    input_map: Res<InputMap>,
    time: Res<Time>,
    mut update_camera: EventWriter<SetCamera>,
) {
    // Handle the sprint modifier.
    if keyboard_input.just_pressed(input_map.free_camera.sprint_modifier) {
        free_camera_state.is_sprinting = true;
    } else if keyboard_input.just_released(input_map.free_camera.sprint_modifier) {
        free_camera_state.is_sprinting = false;
    }

    // Handle pitch and yaw.
    let sensitivity = free_camera_state.sensitivity;
    let dt = time.delta_secs();
    if mouse_input.pressed(MouseButton::Right) {
        for event in evr_mouse_motion.read() {
            free_camera_state.pitch -= event.delta.y * sensitivity * dt;
            free_camera_state.yaw -= event.delta.x * sensitivity * dt;

            // Make sure to clamp the pitch to avoid gimbal lock!
            free_camera_state.pitch = free_camera_state
                .pitch
                .clamp(-consts::PI / 2.1, consts::PI / 2.1);
        }
    }

    // Handle movement.
    let forward = free_camera_state.forward();
    let up = Vec3::Z;
    let right = forward.cross(up).normalize();
    let speed = free_camera_state.speed(dt);
    if keyboard_input.pressed(input_map.free_camera.forward) {
        free_camera_state.translation += forward * speed;
    }
    if keyboard_input.pressed(input_map.free_camera.backward) {
        free_camera_state.translation -= forward * speed;
    }
    if keyboard_input.pressed(input_map.free_camera.left) {
        free_camera_state.translation -= right * speed;
    }
    if keyboard_input.pressed(input_map.free_camera.right) {
        free_camera_state.translation += right * speed;
    }
    if keyboard_input.pressed(input_map.free_camera.up) {
        free_camera_state.translation += up * speed;
    }
    if keyboard_input.pressed(input_map.free_camera.down) {
        free_camera_state.translation -= up * speed;
    }

    // Request a camera update using the SetCamera event.
    update_camera.write(free_camera_state.set_camera());
}
