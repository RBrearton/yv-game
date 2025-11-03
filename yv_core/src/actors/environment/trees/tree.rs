use crate::prelude::*;

/// # Tree
/// A tree is a type of environment object that can be chopped down.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(ActorLike)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(Choppable)]
pub enum Tree {
    Birch(trees::Birch),
    Oak(trees::Oak),
    Willow(trees::Willow),
}
