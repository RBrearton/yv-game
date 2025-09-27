use bevy::prelude::*;

use crate::lighting::well_known_terms::*;

#[derive(Resource, Clone, Debug)]
pub struct LightingConfig {
    pub shadows_enabled: bool,
    pub affect_lightmapped_mesh_diffuse: bool,
    pub num_shadow_cascades: usize,
    pub shadow_map_near_z: f32,
    pub shadow_depth_bias: f32,
    pub shadow_normal_bias: f32,
    pub directional_shadow_map_size: usize,
}
impl Default for LightingConfig {
    fn default() -> Self {
        Self {
            shadows_enabled: true,
            affect_lightmapped_mesh_diffuse: true,
            num_shadow_cascades: DEFAULT_NUM_SHADOW_CASCADES,
            shadow_map_near_z: DEFAULT_SHADOW_MAP_NEAR_Z,
            shadow_depth_bias: DEFAULT_SHADOW_DEPTH_BIAS,
            shadow_normal_bias: DEFAULT_SHADOW_NORMAL_BIAS,
            directional_shadow_map_size: DEFAULT_DIRECTIONAL_SHADOW_MAP_SIZE,
        }
    }
}
