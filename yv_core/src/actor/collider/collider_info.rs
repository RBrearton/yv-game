use super::{cuboid_collider::*, cylinder_collider::*, sphere_collider::*};

/// # Collider info
/// Information required to build a collider for any actor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColliderInfo {
    Cuboid(CuboidCollider),
    Cylinder(CylinderCollider),
    Sphere(SphereCollider),
}
