/// # Massive
/// The trait that must be implemented by anything that has a mass.
pub trait Massive {
    /// # Mass
    /// Returns the mass of the object.
    fn mass(&self) -> f32;
}
