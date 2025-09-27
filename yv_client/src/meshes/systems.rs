use bevy::prelude::*;

use crate::meshes::Meshes;
use crate::well_known_terms::{meshes, TERRAIN_MESH_THICKNESS, TERRAIN_MESH_WIDTH};

pub(super) fn init_meshes(
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut meshes_resource: ResMut<Meshes>,
) {
    // The terrain mesh is a flat square mesh that will be used as the ground in the world.
    let terrain_mesh_handle = mesh_assets.add(Cuboid::new(
        TERRAIN_MESH_WIDTH,
        TERRAIN_MESH_WIDTH,
        TERRAIN_MESH_THICKNESS,
    ));
    meshes_resource.terrain = terrain_mesh_handle;

    // Load the player mesh.
    // This capsule uses the player's defined dimensions from constants.
    let player_mesh_handle =
        mesh_assets.add(Capsule3d::new(meshes::HUMAN_RADIUS, meshes::HUMAN_HEIGHT));
    meshes_resource.player = player_mesh_handle;

    // Load the tree meshes.
    let tree_trunk_mesh_handle = mesh_assets.add(Cylinder::new(
        meshes::TREE_TRUNK_RADIUS,
        meshes::TREE_TRUNK_HEIGHT,
    ));
    let tree_top_mesh_handle = mesh_assets.add(Sphere::new(meshes::TREE_TOP_RADIUS));
    meshes_resource.tree_trunk = tree_trunk_mesh_handle;
    meshes_resource.tree_top = tree_top_mesh_handle;
}
