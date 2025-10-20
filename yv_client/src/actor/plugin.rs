use bevy::prelude::*;

use crate::actor::{events::*, systems::*};

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
