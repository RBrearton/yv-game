use bevy::pbr::CascadeShadowConfig;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::prelude::*;

use crate::lighting::events::*;
use crate::lighting::resources::*;

pub(super) fn spawn_directional_light(
    config: Res<LightingConfig>,
    mut evr_add_directional_light: EventReader<AddDirectionalLight>,
    mut commands: Commands,
) {
    for event in evr_add_directional_light.read() {
        let mut transform = Transform::default();
        transform.rotate_z(event.yaw);
        transform.rotate_y(event.pitch);
        commands.spawn((
            DirectionalLight {
                illuminance: event.intensity,
                shadows_enabled: config.shadows_enabled,
                affects_lightmapped_mesh_diffuse: config.affect_lightmapped_mesh_diffuse,
                shadow_depth_bias: config.shadow_depth_bias,
                shadow_normal_bias: config.shadow_normal_bias,
                color: event.color,
            },
            CascadeShadowConfigBuilder {
                num_cascades: config.num_shadow_cascades,
                ..default()
            }
            .build(),
            transform,
        ));
    }
}

pub(super) fn spawn_point_light(
    config: Res<LightingConfig>,
    mut evr_add_point_light: EventReader<AddPointLight>,
    mut commands: Commands,
) {
    for event in evr_add_point_light.read() {
        commands.spawn((
            PointLight {
                intensity: event.intensity,
                shadows_enabled: config.shadows_enabled,
                affects_lightmapped_mesh_diffuse: config.affect_lightmapped_mesh_diffuse,
                shadow_depth_bias: config.shadow_depth_bias,
                shadow_normal_bias: config.shadow_normal_bias,
                shadow_map_near_z: config.shadow_map_near_z,
                color: event.color,
                range: event.range,
                radius: event.radius,
            },
            Transform::from_translation(event.position),
        ));
    }
}

/// This system updates lighting after a lighting config change.
pub(super) fn update_lighting_on_config_change(
    lighting: Res<LightingConfig>,
    mut directional_light_shadow_map: ResMut<DirectionalLightShadowMap>,
    mut point_lights: Query<&mut PointLight>,
    mut directional_lights: Query<&mut DirectionalLight>,
    mut cascade_shadows: Query<&mut CascadeShadowConfig>,
) {
    if lighting.is_changed() {
        // Set the shadow map size for directional lights.
        directional_light_shadow_map.size = lighting.directional_shadow_map_size;

        // Update lighting config for all point lights.
        for mut point_light in point_lights.iter_mut() {
            point_light.shadows_enabled = lighting.shadows_enabled;
            point_light.affects_lightmapped_mesh_diffuse = lighting.affect_lightmapped_mesh_diffuse;
            point_light.shadow_depth_bias = lighting.shadow_depth_bias;
            point_light.shadow_normal_bias = lighting.shadow_normal_bias;
        }

        // Update lighting config for all directional lights.
        for mut directional_light in directional_lights.iter_mut() {
            directional_light.shadows_enabled = lighting.shadows_enabled;
            directional_light.affects_lightmapped_mesh_diffuse =
                lighting.affect_lightmapped_mesh_diffuse;
            directional_light.shadow_depth_bias = lighting.shadow_depth_bias;
            directional_light.shadow_normal_bias = lighting.shadow_normal_bias;
        }

        // Rebuild the cascade shadows.
        for mut cascade_shadow in cascade_shadows.iter_mut() {
            *cascade_shadow = CascadeShadowConfigBuilder {
                num_cascades: lighting.num_shadow_cascades,
                ..default()
            }
            .build();
        }
    }
}
