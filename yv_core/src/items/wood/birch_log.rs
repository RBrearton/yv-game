use crate::prelude::*;

/// # Birch log
/// A birch log is a type of wood that can be used to craft items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BirchLog {}

impl Describable for BirchLog {
    fn description(&self) -> &str {
        well_known_terms::descriptions::wood::BIRCH_LOG
    }
}

impl HasDisplayName for BirchLog {
    fn display_name(&self) -> &str {
        well_known_terms::wood::BIRCH_LOG
    }
}
