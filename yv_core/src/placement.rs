use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// # Placement
/// A struct that represents the placement of an object in the game world.
/// This includes the position and rotation of the object.
/// As the game is fundamentally something like 2.5D, we only need to store the rotation around the
/// Z axis for gameplay purposes.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Placement {
    /// The object's position in the game world.
    pub position: Vector3,

    /// The object's rotation around the Z axis.
    pub rotation: f32,
}
