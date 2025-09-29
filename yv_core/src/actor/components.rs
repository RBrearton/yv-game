use crate::well_known_terms::actors;
use bevy::prelude::*;

#[derive(Component, Copy, Clone, Default, Debug, Eq, PartialEq)]
#[require(Transform, InheritedVisibility)]
pub enum ActorType {
    #[default]
    Tree,
    Player,
}
impl ActorType {
    /// The radius of the actor's cylinder collider mesh.
    pub fn radius(self) -> f32 {
        match self {
            ActorType::Tree => actors::TREE_TOTAL_RADIUS,
            ActorType::Player => actors::HUMAN_TOTAL_RADIUS,
        }
    }

    /// The height of the actor's cylinder collider mesh.
    pub fn height(self) -> f32 {
        match self {
            ActorType::Tree => actors::TREE_TOTAL_HEIGHT,
            ActorType::Player => actors::HUMAN_TOTAL_HEIGHT,
        }
    }
}
