//! # Actor
//! This module contains all core functionality related to actors in the game.
//! An actor is something visible and interactive in the game world.
//! All Characters are actors, but the inverse is not true.

use bevy::prelude::*;

pub struct ActorPlugin;
impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnActor>();
        app.add_event::<DespawnActor>();
        app.add_event::<ActorSpawned>();

        app.add_systems(
            Update,
            (
                spawn_actor_system.run_if(on_event::<SpawnActor>),
                despawn_actor_system.run_if(on_event::<DespawnActor>),
            ),
        );
    }
}

fn spawn_actor_system(
    mut commands: Commands,
    mut actor_events: EventReader<SpawnActor>,
    mut evw_actor_spawned: EventWriter<ActorSpawned>,
) {
    for event in actor_events.read() {
        let entity = commands.spawn((
            event.actor_type.clone(),
            Transform::from_translation(event.position),
        ));
        evw_actor_spawned.write(ActorSpawned {
            actor_type: event.actor_type.clone(),
            entity: entity.id(),
        });
    }
}

fn despawn_actor_system(mut commands: Commands, mut actor_events: EventReader<DespawnActor>) {
    for event in actor_events.read() {
        commands.entity(event.entity).despawn();
    }
}

#[derive(Component, Clone, Debug, Eq, PartialEq)]
#[require(Transform)]
pub enum ActorType {
    Tree,
    Player,
}

#[derive(Event, Clone, Debug)]
pub struct SpawnActor {
    pub actor_type: ActorType,
    pub position: Vec3,
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
