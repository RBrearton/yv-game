use bevy::prelude::*;

#[derive(Event, Clone, Debug)]
pub struct AddDirectionalLight {
    pub yaw: f32,
    pub pitch: f32,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Event, Clone, Debug)]
pub struct AddPointLight {
    pub position: Vec3,
    pub range: f32,
    pub radius: f32,
    pub color: Color,
    pub intensity: f32,
}
