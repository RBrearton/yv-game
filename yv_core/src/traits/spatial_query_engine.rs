use crate::prelude::*;

/// # Spatial query engine
/// The trait that must be implemented by anything that can be used as a spatial query engine.
pub trait SpatialQueryEngine {
    /// # Entity type
    /// The type of entity that this spatial query engine can query.
    type EntityType: Identifiable;

    /// # Get in range
    /// Returns all of the entities that are within the requested range.
    fn get_in_range(&self, range: f32, position: Vector2, out: &mut Vec<Self::EntityType>);
}
