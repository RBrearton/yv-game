use crate::prelude::*;

/// # Mining node
/// A mining node is a type of environment object that can be mined for resources.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(ActorLike)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(Mineable)]
pub enum MiningNode {
    Copper(mining_nodes::CopperNode),
    Iron(mining_nodes::IronNode),
}
