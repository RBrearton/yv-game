use bevy::prelude::*;

use super::{
    geometry_info::{Geometry, GeometryType},
    shapes::Box,
};

/// A trait that should be implemented by any physical object.
/// Really handy for debugging etc. to be able to drop a bounding box over the top of an object.
pub trait HasBoundingBox {
    fn bounding_box(&self) -> Box;
    fn center(&self) -> Vec3 {
        self.bounding_box().center()
    }
}

/// A trait that should be implemented by anything associated with a volume.
pub trait HasVolume {
    fn volume(&self) -> f32;
}

/// A trait that should be implemented by any component of an actor's geometry.
/// This just helps give us a uniform API, and helps unwind/unnest heavily nested enums.
pub trait HasGeometryType {
    fn geometry_type(&self) -> GeometryType;
}

/// A trait that must be satisfied by anything physical in the game.
pub trait Physical {
    /// All geometry components that make up the physical object.
    fn geometry(&self) -> Vec<Geometry>;
}
