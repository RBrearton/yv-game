use crate::prelude::*;

/// # Oak log
/// An oak log is a type of wood that can be used to craft items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OakLog {}

impl Describable for OakLog {
    fn description(&self) -> &str {
        well_known_terms::descriptions::wood::OAK_LOG
    }
}

impl HasDisplayName for OakLog {
    fn display_name(&self) -> &str {
        well_known_terms::wood::OAK_LOG
    }
}
