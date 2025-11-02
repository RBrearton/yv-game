use crate::prelude::*;

/// # Copper node
/// A copper node is a type of mining node that can be mined for copper ore.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopperNode {}

impl Describable for CopperNode {
    fn description(&self) -> &str {
        well_known_terms::descriptions::mining_nodes::COPPER
    }
}

impl HasDisplayName for CopperNode {
    fn display_name(&self) -> &str {
        well_known_terms::mining_nodes::COPPER
    }
}
