use bevy::prelude::*;

use super::ActorType;
use crate::geometry::{Geometry, Physical};

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
impl Physical for Actor {
    fn geometry(&self) -> Vec<Geometry> {
        match self.actor_type {
            ActorType::Tree => {
                // The tree parameters; it's a cylinder with a sphere on top.
                let trunk_height = 2.0;
                let trunk_radius = 0.8;
                let top_radius = 2.5;

                // The offsets to the COM of the trunk/top.
                let trunk_transform =
                    Transform::from_translation(Vec3::new(0.0, 0.0, trunk_height / 2.0));
                let top_transform =
                    Transform::from_translation(Vec3::new(0.0, 0.0, trunk_height + top_radius));

                // The geometry of the trunk and top.
                let trunk = Geometry::new_cylinder(trunk_radius, trunk_height, trunk_transform);
                let top = Geometry::new_sphere(top_radius, top_transform);
                vec![trunk, top]
            }
            ActorType::Player => {
                let radius = 0.3;
                let height = 1.8;
                let transform = Transform::from_translation(Vec3::new(0.0, 0.0, height / 2.0));
                vec![Geometry::new_capsule(radius, height, transform)]
            }
        }
    }
}
