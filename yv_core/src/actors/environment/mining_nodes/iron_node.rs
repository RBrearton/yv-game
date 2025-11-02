use crate::prelude::*;

/// # Iron node
/// An iron node is a type of mining node that can be mined for iron ore.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(ActorLike)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
pub struct IronNode {
    pub core: ActorCore,
}

impl Describable for IronNode {
    fn description(&self) -> &str {
        well_known_terms::descriptions::mining_nodes::IRON
    }
}

impl HasDisplayName for IronNode {
    fn display_name(&self) -> &str {
        well_known_terms::mining_nodes::IRON
    }
}
