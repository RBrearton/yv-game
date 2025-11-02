use crate::prelude::*;

/// # Birch
/// A birch tree is a type of tree that can be chopped down.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Birch {}

impl Describable for Birch {
    fn description(&self) -> &str {
        well_known_terms::descriptions::trees::BIRCH
    }
}

impl HasDisplayName for Birch {
    fn display_name(&self) -> &str {
        well_known_terms::trees::BIRCH
    }
}
