use bevy::prelude::*;

use super::components::*;
use super::events::*;

pub(super) fn attach_player_system(
    mut commands: Commands,
    mut evr_attach_player: EventReader<AttachPlayer>,
) {
    for event in evr_attach_player.read() {
        commands.entity(event.entity).insert(Player);
    }
}

pub(super) fn detach_player_system(
    mut commands: Commands,
    mut evr_detach_player: EventReader<DetachPlayer>,
) {
    for event in evr_detach_player.read() {
        commands.entity(event.entity).remove::<Player>();
    }
}
