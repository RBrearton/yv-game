use crate::prelude::*;

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
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}
