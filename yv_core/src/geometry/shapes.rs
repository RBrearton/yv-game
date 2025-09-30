use avian3d::math::PI;
use bevy::prelude::*;

use super::traits::*;

/// A box shape.
/// This is a 3D cuboid.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Box {
    size: Vec3,
}
impl Box {
    pub fn new(size: Vec3) -> Self {
        Self { size }
    }
    pub fn new_scalar(size_x: f32, size_y: f32, size_z: f32) -> Self {
        Self::new(Vec3::new(size_x, size_y, size_z))
    }
    pub fn get_size(&self) -> Vec3 {
        self.size
    }
    pub fn size_x(&self) -> f32 {
        self.size.x
    }
    pub fn size_y(&self) -> f32 {
        self.size.y
    }
    pub fn size_z(&self) -> f32 {
        self.size.z
    }
    pub fn center(&self) -> Vec3 {
        self.size / 2.0
    }
}
impl HasBoundingBox for Box {
    fn bounding_box(&self) -> Box {
        // A box is its own bounding box.
        *self
    }
}
impl HasVolume for Box {
    fn volume(&self) -> f32 {
        self.size.x * self.size.y * self.size.z
    }
}

/// A capsule shape.
/// This is a 3D cylinder with a hemispherical end on each side.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Capsule {
    radius: f32,
    height: f32,
}
impl Capsule {
    pub fn new(radius: f32, height: f32) -> Self {
        Self { radius, height }
    }
}
impl HasBoundingBox for Capsule {
    fn bounding_box(&self) -> Box {
        let width = 2.0 * self.radius;
        let height = self.height + 2.0 * self.radius;
        Box::new(Vec3::new(width, width, height))
    }
}
impl HasVolume for Capsule {
    fn volume(&self) -> f32 {
        let hemisphere_volume = 2. / 3. * PI * self.radius.powi(3);
        let cylinder_volume = PI * self.radius.powi(2) * self.height;

        // Volume of both hemispheres plus the volume of the cylinder.
        hemisphere_volume * 2.0 + cylinder_volume
    }
}

/// A simple sphere shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    radius: f32,
}
impl Sphere {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn diameter(&self) -> f32 {
        2.0 * self.radius
    }
}
impl HasBoundingBox for Sphere {
    fn bounding_box(&self) -> Box {
        let diameter = self.diameter();
        Box::new(Vec3::new(diameter, diameter, diameter))
    }
}
impl HasVolume for Sphere {
    fn volume(&self) -> f32 {
        (4.0 / 3.0) * PI * self.radius.powi(3)
    }
}

/// A simple cylinder shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylinder {
    radius: f32,
    height: f32,
}
impl Cylinder {
    pub fn new(radius: f32, height: f32) -> Self {
        Self { radius, height }
    }
    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn height(&self) -> f32 {
        self.height
    }
}
impl HasBoundingBox for Cylinder {
    fn bounding_box(&self) -> Box {
        let diameter = 2.0 * self.radius;
        let height = self.height;
        Box::new(Vec3::new(diameter, diameter, height))
    }
}
impl HasVolume for Cylinder {
    fn volume(&self) -> f32 {
        PI * self.radius.powi(2) * self.height
    }
}
