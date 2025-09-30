/// # Cylinder collider
/// Contains data required to define a cylinder collider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CylinderCollider {
    pub radius: f32,
    pub height: f32,
}
