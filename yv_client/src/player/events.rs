use bevy::prelude::*;

/// Event to attach the player component to an entity.
/// This is very handy when debugging etc. because it lets us dynamically choose which entity we
/// want to move around,
#[derive(Event, Clone, Debug, PartialEq, Hash)]
pub struct AttachPlayer {
    pub entity: Entity,
}

/// Event to detach the player component from an entity.
#[derive(Event, Clone, Debug, PartialEq, Hash)]
pub struct DetachPlayer {
    pub entity: Entity,
}
