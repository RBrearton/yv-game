use crate::prelude::*;

/// # Willow log
/// A willow log is a type of wood that can be used to craft items.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WillowLog {}

impl Describable for WillowLog {
    fn description(&self) -> &str {
        well_known_terms::descriptions::wood::WILLOW_LOG
    }
}

impl HasDisplayName for WillowLog {
    fn display_name(&self) -> &str {
        well_known_terms::wood::WILLOW_LOG
    }
}
