use bevy::prelude::*;

use super::{components::*, events::*};

pub(super) fn spawn_actor_system(
    mut commands: Commands,
    mut actor_events: EventReader<SpawnActor>,
    mut evw_actor_spawned: EventWriter<ActorSpawned>,
) {
    for event in actor_events.read() {
        let entity = commands.spawn((Actor::new(event.actor_type), event.transform));
        evw_actor_spawned.write(ActorSpawned {
            actor_type: event.actor_type,
            entity: entity.id(),
        });
    }
}

pub(super) fn despawn_actor_system(
    mut commands: Commands,
    mut actor_events: EventReader<DespawnActor>,
) {
    for event in actor_events.read() {
        commands.entity(event.entity).despawn();
    }
}
