/// # Cuboid collider
/// Contains data required to define a cuboid collider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CuboidCollider {
    pub x_size: f32,
    pub y_size: f32,
    pub z_size: f32,
}
