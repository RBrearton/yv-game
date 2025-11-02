use crate::prelude::*;

/// # Copper ore
/// The main ore that we extract when mining from a copper node.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CopperOre {}

impl Describable for CopperOre {
    fn description(&self) -> &str {
        well_known_terms::descriptions::ores::COPPER
    }
}

impl HasDisplayName for CopperOre {
    fn display_name(&self) -> &str {
        well_known_terms::ores::COPPER
    }
}
