use bevy::prelude::*;

/// A marker component that indicates that we want to highlight an entity when we hover over it.
#[derive(Component)]
#[require(Pickable)]
pub struct HighlightOnHover;

/// A simple marker component that marks an entity as a child of a highlighted entity.
#[derive(Component)]
pub(super) struct OutlineChild;
