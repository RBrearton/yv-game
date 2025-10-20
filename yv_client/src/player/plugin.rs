use bevy::prelude::*;

use super::events::*;
use super::systems::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttachPlayer>();
        app.add_event::<DetachPlayer>();

        app.add_systems(Update, (detach_player_system, attach_player_system).chain());
    }
}
