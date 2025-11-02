use crate::prelude::*;

/// # Iron ore
/// The main ore that we extract when mining from an iron node.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IronOre {}

impl Describable for IronOre {
    fn description(&self) -> &str {
        well_known_terms::descriptions::ores::IRON
    }
}

impl HasDisplayName for IronOre {
    fn display_name(&self) -> &str {
        well_known_terms::ores::IRON
    }
}
