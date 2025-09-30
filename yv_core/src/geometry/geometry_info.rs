use bevy::transform::components::Transform;

use super::traits::*;

use super::shapes::*;

/// The type of geometry that the object has.
/// This is the local geometry of the object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GeometryType {
    Box(Box),
    Capsule(Capsule),
    Sphere(Sphere),
    Cylinder(Cylinder),
}
impl HasVolume for GeometryType {
    fn volume(&self) -> f32 {
        match self {
            GeometryType::Box(box_) => box_.volume(),
            GeometryType::Capsule(capsule) => capsule.volume(),
            GeometryType::Sphere(sphere) => sphere.volume(),
            GeometryType::Cylinder(cylinder) => cylinder.volume(),
        }
    }
}
impl HasBoundingBox for GeometryType {
    fn bounding_box(&self) -> Box {
        match self {
            GeometryType::Box(box_) => box_.bounding_box(),
            GeometryType::Capsule(capsule) => capsule.bounding_box(),
            GeometryType::Sphere(sphere) => sphere.bounding_box(),
            GeometryType::Cylinder(cylinder) => cylinder.bounding_box(),
        }
    }
}
impl HasGeometryType for GeometryType {
    fn geometry_type(&self) -> GeometryType {
        *self
    }
}

/// The geometry of the object.
/// This contains the local geometry type, as well as the relative transform of the geometry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Geometry {
    geometry_type: GeometryType,
    pub relative_transform: Transform,
}

impl Geometry {
    pub fn new(geometry_type: GeometryType, relative_transform: Transform) -> Self {
        Self {
            geometry_type,
            relative_transform,
        }
    }
}
impl HasGeometryType for Geometry {
    fn geometry_type(&self) -> GeometryType {
        self.geometry_type
    }
}
impl HasBoundingBox for Geometry {
    fn bounding_box(&self) -> Box {
        self.geometry_type.bounding_box()
    }
}
impl HasVolume for Geometry {
    fn volume(&self) -> f32 {
        self.geometry_type.volume()
    }
}
