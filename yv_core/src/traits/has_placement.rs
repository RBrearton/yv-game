use crate::prelude::*;

/// # Has placement
/// The trait that must be implemented by anything that exists in game world space.
#[delegatable_trait]
pub trait HasPlacement {
    /// # Placement
    /// Returns a reference to the placement of the object.
    fn placement(&self) -> &Placement;

    /// # Placement mut
    /// Returns a mutable reference to the placement of the object.
    fn placement_mut(&mut self) -> &mut Placement;

    /// # Position
    /// Returns the position of the object.
    fn position(&self) -> Vector2 {
        self.placement().position
    }

    /// # Distance to
    /// Returns the distance between the object and the given position.
    fn distance_to(&self, position: Vector2) -> f32 {
        self.position().distance_to(position)
    }

    /// # Position mut
    /// Returns a mutable reference to the position of the object.
    fn position_mut(&mut self) -> &mut Vector2 {
        &mut self.placement_mut().position
    }

    /// # Rotation
    /// Returns the rotation of the object.
    fn rotation(&self) -> f32 {
        self.placement().rotation
    }

    /// # Rotation mut
    /// Returns a mutable reference to the rotation of the object.
    fn rotation_mut(&mut self) -> &mut f32 {
        &mut self.placement_mut().rotation
    }

    /// # Set position
    /// Sets the position of the object.
    fn set_position(&mut self, position: Vector2) {
        *self.position_mut() = position;
    }

    /// # Set rotation
    /// Sets the rotation of the object.
    fn set_rotation(&mut self, rotation: f32) {
        *self.rotation_mut() = rotation;
    }

    /// # Set placement
    /// Sets the placement of the object.
    fn set_placement(&mut self, placement: Placement) {
        *self.placement_mut() = placement;
    }
}
