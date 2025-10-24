/// # Describable
/// The trait that must be implemented by anything that can be described.
pub trait Describable {
    /// # Description
    /// Returns the description of the object.
    /// This is used extensively by the client to produce things like tooltips, hover text, etc.
    fn description(&self) -> &str;
}
