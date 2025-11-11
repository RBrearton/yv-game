use crate::prelude::*;

/// # Vector2
/// A 2D vector.
/// This is a simple struct that contains two floating point numbers.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// # Distance to
    /// Returns the distance between the vector and the given position.
    pub fn distance_to(&self, other: Vector2) -> f32 {
        self.distance_to_sq(other).sqrt()
    }

    /// # Distance to squared
    /// Returns the squared distance between the vector and the given position.
    pub fn distance_to_sq(&self, other: Vector2) -> f32 {
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }
}
