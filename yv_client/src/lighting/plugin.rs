use bevy::prelude::*;

use super::events::*;
use super::resources::*;
use super::system_sets::LightingUpdateSystems;
use super::systems::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddDirectionalLight>();
        app.add_event::<AddPointLight>();
        app.init_resource::<LightingConfig>();

        app.add_systems(
            Update,
            (
                update_lighting_on_config_change,
                spawn_directional_light,
                spawn_point_light,
            )
                .in_set(LightingUpdateSystems),
        );
    }
}
