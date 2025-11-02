use crate::prelude::*;

/// # Has display name
/// The trait that must be implemented by anything that has a display name.
#[delegatable_trait]
pub trait HasDisplayName {
    /// # Display name
    /// Returns the display name of the object.
    /// Unlike an Identity, a DisplayName doesn't need to be unique, and it should focus on being
    /// human readable and understandable.
    fn display_name(&self) -> &str;
}
