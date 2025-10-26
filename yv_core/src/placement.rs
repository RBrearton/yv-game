use crate::prelude::*;

/// # Placement
/// A struct that represents the placement of an object in the game world.
/// This includes the position and rotation of the object.
/// As the game is fundamentally 2.5D, we only need to store the 2D position and rotation.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Placement {
    /// The object's position in the game world.
    pub position: Vector2,

    /// The object's rotation (in radians).
    pub rotation: f32,
}
