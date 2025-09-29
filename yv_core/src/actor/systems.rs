use bevy::prelude::*;

use crate::actor::events::*;

pub fn spawn_actor_system(
    mut commands: Commands,
    mut actor_events: EventReader<SpawnActor>,
    mut evw_actor_spawned: EventWriter<ActorSpawned>,
) {
    for event in actor_events.read() {
        let entity = commands.spawn((event.actor_type, event.transform));
        evw_actor_spawned.write(ActorSpawned {
            actor_type: event.actor_type,
            entity: entity.id(),
        });
    }
}

pub fn despawn_actor_system(mut commands: Commands, mut actor_events: EventReader<DespawnActor>) {
    for event in actor_events.read() {
        commands.entity(event.entity).despawn();
    }
}
