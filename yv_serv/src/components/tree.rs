use crate::prelude::*;

/// # Tree
/// A component that identifies an actor as being a tree.
/// This is a marker component, but it also contains the type of tree so that various statistics
/// associated with the tree can be efficiently stored/retrieved.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Component)]
pub struct Tree {
    pub tree_type: TreeType,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum TreeType {
    Birch,
    Oak,
    Willow,
}
