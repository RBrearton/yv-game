use bevy::prelude::*;

/// # Meshes
/// This resource holds handles to all custom meshes that we want to use in the game.
#[derive(Resource, Reflect, Default)]
pub struct Meshes {
    /// The handle to the world quad mesh.
    pub terrain: Handle<Mesh>,

    /// The handle to the player mesh.
    pub player: Handle<Mesh>,

    /// The handle to the tree trunk mesh.
    pub tree_trunk: Handle<Mesh>,

    /// The handle to the tree top mesh.
    pub tree_top: Handle<Mesh>,
}
