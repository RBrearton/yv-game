use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Placement {
    pub position: Vector3,
    pub rotation: f32,
}
