use bevy::prelude::*;

use super::ActorType;

#[derive(Event, Clone, Debug)]
pub struct SpawnActor {
    pub actor_type: ActorType,
    pub transform: Transform,
}

#[derive(Event, Clone, Debug)]
pub struct DespawnActor {
    pub entity: Entity,
}

#[derive(Event, Clone, Debug)]
pub struct ActorSpawned {
    pub actor_type: ActorType,
    pub entity: Entity,
}
