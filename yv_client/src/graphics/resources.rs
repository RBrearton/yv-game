use bevy::prelude::*;

use super::well_known_terms;

#[derive(Resource)]
pub struct HoverHighlightConfig {
    pub highlight_color: Color,
    pub depth_bias: f32,
    pub scale_factor: f32,
}
impl Default for HoverHighlightConfig {
    fn default() -> Self {
        Self {
            highlight_color: well_known_terms::DEFAULT_HIGHLIGHT_COLOR,
            depth_bias: well_known_terms::DEFAULT_HIGHLIGHT_DEPTH_BIAS,
            scale_factor: well_known_terms::DEFAULT_HIGHLIGHT_SCALE_FACTOR,
        }
    }
}
