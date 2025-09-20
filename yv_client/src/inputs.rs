use bevy::prelude::*;

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputMap>();
    }
}

#[derive(Resource, Default)]
pub struct InputMap {
    pub free_camera: FreeCameraControls,
}

pub struct FreeCameraControls {
    // Fundamental movement keys.
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,

    // Modifier keys.
    pub sprint_modifier: KeyCode,
}
impl Default for FreeCameraControls {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::Space,
            down: KeyCode::KeyZ,
            sprint_modifier: KeyCode::ShiftLeft,
        }
    }
}
