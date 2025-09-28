use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy::render::render_resource::Face;

use super::{components::*, resources::*};

/// Highlights all pickable entities with a HighlightOnHover component when we hover over them.
pub(super) fn highlight_on_hover(
    trigger: Trigger<Pointer<Over>>,
    meshes: Query<&Mesh3d, With<HighlightOnHover>>,
    config: Res<HoverHighlightConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    // If the target doesn't have a mesh with a HighlightOnHover component, do nothing.
    let Ok(mesh) = meshes.get(trigger.target()) else {
        return;
    };

    let mat = materials.add(StandardMaterial {
        base_color: config.highlight_color,
        unlit: true,
        cull_mode: Some(Face::Front), // draw backfaces (classic outline trick)
        depth_bias: config.depth_bias, // reduce z-fighting
        ..default()
    });

    let child = commands
        .spawn((
            Mesh3d(mesh.0.clone()),
            MeshMaterial3d(mat),
            Transform::from_scale(Vec3::splat(config.scale_factor)),
            OutlineChild,
        ))
        .id();

    commands.entity(trigger.target()).add_child(child);
}

/// Unhighlights highlighted entities when we stop hovering over them.
pub(super) fn unhighlight_on_out(
    trigger: Trigger<Pointer<Out>>,
    children: Query<&Children>,
    mut commands: Commands,
    marks: Query<(), With<OutlineChild>>,
) {
    if let Ok(cs) = children.get(trigger.target()) {
        for &c in cs {
            if marks.get(c).is_ok() {
                commands.entity(c).despawn();
            }
        }
    }
}
