use bevy::prelude::*;

use super::ActorType;

#[derive(Component, Copy, Clone, Default, Debug, Eq, PartialEq)]
#[require(Transform, InheritedVisibility)]
pub struct Actor {
    actor_type: ActorType,
}
impl Actor {
    pub fn new(actor_type: ActorType) -> Self {
        Self { actor_type }
    }
}
