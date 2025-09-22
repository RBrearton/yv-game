use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};
use egui::DragValue;
use yv_core::{
    actor::{ActorType, SpawnActor},
    terrain::{AddTerrainChunk, Biome, ChunkType, ProcedurallyGenerateTerrain, TerrainIndex},
};

use crate::{
    camera::{CameraMode, FreeCameraState},
    scenes::YvScene,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IngameDebugData>();
        app.add_systems(
            EguiPrimaryContextPass,
            (
                login_window.run_if(in_state(YvScene::MainMenu)),
                ingame_debug_window.run_if(in_state(YvScene::Game)),
            ),
        );
    }
}

/// Extension trait for `egui::Ui` to add some custom methods.
/// This is used extensively to keep the UI code somewhat modular.
pub(super) trait UiExtensions {
    fn big_button(&mut self, text: &str) -> egui::Response;

    /// The entire terrain chunk input section of the ingame debug window.
    fn terrain_chunk_input_section(
        &mut self,
        ingame_debug_data: &mut IngameDebugData,
        add_terrain: &mut EventWriter<AddTerrainChunk>,
        generate_terrain: &mut EventWriter<ProcedurallyGenerateTerrain>,
    );

    fn actor_input_section(
        &mut self,
        data: &mut IngameDebugData,
        add_actor: &mut EventWriter<SpawnActor>,
    );

    /// The entire camera section of the ingame debug window.
    fn camera_section(
        &mut self,
        camera_state: &mut FreeCameraState,
        camera_mode: Res<State<CameraMode>>,
        next_camera_mode: ResMut<NextState<CameraMode>>,
    );
}
impl UiExtensions for egui::Ui {
    fn big_button(&mut self, text: &str) -> egui::Response {
        self.add_sized(
            [self.available_width() * 0.8, 36.0],
            egui::Button::new(egui::RichText::new(text).size(16.0).strong()),
        )
    }

    fn camera_section(
        &mut self,
        free_camera_state: &mut FreeCameraState,
        camera_mode: Res<State<CameraMode>>,
        mut next_camera_mode: ResMut<NextState<CameraMode>>,
    ) {
        self.heading("Camera");
        self.label(format!("Current camera mode: {}", camera_mode.get()));
        self.horizontal(|ui| {
            ui.label("Select camera mode");
            if ui.button("Free").clicked() {
                next_camera_mode.set(CameraMode::Free);
            }
            if ui.button("Follow player").clicked() {
                next_camera_mode.set(CameraMode::FollowPlayer);
            }
        });

        if *camera_mode == CameraMode::Free {
            self.add_space(8.0);
            self.heading("Camera details");
            self.horizontal(|ui| {
                ui.label("X");
                ui.add(DragValue::new(&mut free_camera_state.translation.x));
                ui.label("Y");
                ui.add(DragValue::new(&mut free_camera_state.translation.y));
                ui.label("Z");
                ui.add(DragValue::new(&mut free_camera_state.translation.z));
            });
            self.add_space(4.0);
            self.horizontal(|ui| {
                ui.label("Pitch");
                ui.add(DragValue::new(&mut free_camera_state.pitch));
                ui.label("Yaw");
                ui.add(DragValue::new(&mut free_camera_state.yaw));
                ui.label("Sensitivity");
                ui.add(DragValue::new(&mut free_camera_state.sensitivity));
            });
            self.add_space(4.0);
            self.horizontal(|ui| {
                ui.label("Base speed");
                ui.add(DragValue::new(&mut free_camera_state.base_speed));
                ui.label("Sprint modifier");
                ui.add(DragValue::new(&mut free_camera_state.sprint_modifier));
                ui.label("Is sprinting");
                ui.checkbox(&mut free_camera_state.is_sprinting, "Sprint");
            });
        }
    }

    fn actor_input_section(
        &mut self,
        data: &mut IngameDebugData,
        add_actor: &mut EventWriter<SpawnActor>,
    ) {
        self.heading("Actor");

        self.horizontal(|ui| {
            ui.label("X");
            ui.add(DragValue::new(&mut data.add_actor_x));
            ui.label("Y");
            ui.add(DragValue::new(&mut data.add_actor_y));
            ui.label("Actor type:");
            ui.selectable_value(&mut data.add_actor_actor_type, ActorType::Tree, "Tree");
            ui.selectable_value(&mut data.add_actor_actor_type, ActorType::Player, "Player");
        });
        if self.button("Add actor").clicked() {
            add_actor.write(SpawnActor {
                actor_type: data.add_actor_actor_type,
                transform: Transform::from_translation(Vec3::new(
                    data.add_actor_x,
                    data.add_actor_y,
                    0.0, // Z axis is currently unused.
                )),
            });
        }
    }

    fn terrain_chunk_input_section(
        &mut self,
        data: &mut IngameDebugData,
        add_terrain: &mut EventWriter<AddTerrainChunk>,
        generate_terrain: &mut EventWriter<ProcedurallyGenerateTerrain>,
    ) {
        // Terrain chunk debugging.
        self.heading("Terrain chunk");

        // Batch generation.
        self.horizontal(|ui| {
            ui.label("Generate terrain (start x, start y, end x, end y):");
        });
        self.horizontal(|ui| {
            ui.add(DragValue::new(&mut data.bulk_add_terrain_start_x));
            ui.add(DragValue::new(&mut data.bulk_add_terrain_start_y));
            ui.add(DragValue::new(&mut data.bulk_add_terrain_end_x));
            ui.add(DragValue::new(&mut data.bulk_add_terrain_end_y));
            if ui.button("Generate terrain").clicked() {
                generate_terrain.write(ProcedurallyGenerateTerrain::new_range(
                    data.bulk_add_terrain_start_x,
                    data.bulk_add_terrain_end_x,
                    data.bulk_add_terrain_start_y,
                    data.bulk_add_terrain_end_y,
                ));
            }
        });

        // Single specific chunk generation.
        self.add_space(8.0);
        self.heading("Specific chunk generation");
        self.horizontal(|ui| {
            ui.label("X");
            ui.add(DragValue::new(&mut data.add_terrain_chunk_x));
            ui.label("Y");
            ui.add(DragValue::new(&mut data.add_terrain_chunk_y));
        });
        self.horizontal(|ui| {
            ui.label("Chunk type:");
            ui.selectable_value(
                &mut data.add_terrain_chunk_chunk_type,
                ChunkType::Grass,
                "Grass",
            );
            ui.selectable_value(
                &mut data.add_terrain_chunk_chunk_type,
                ChunkType::Water,
                "Water",
            );
        });
        self.horizontal(|ui| {
            ui.label("Biome:");
            ui.selectable_value(&mut data.add_terrain_chunk_biome, Biome::Meadow, "Meadow");
            ui.selectable_value(&mut data.add_terrain_chunk_biome, Biome::Snow, "Snow");
        });
        if self.button("Add terrain chunk").clicked() {
            add_terrain.write(AddTerrainChunk::new(
                TerrainIndex::new(data.add_terrain_chunk_x, data.add_terrain_chunk_y),
                data.add_terrain_chunk_chunk_type,
                data.add_terrain_chunk_biome,
            ));
        }
    }
}

/// This system produces the main menu window.
/// This is called login_window because the intent is, in the future, to have a full login system.
fn login_window(mut contexts: EguiContexts, mut scene_state: ResMut<NextState<YvScene>>) -> Result {
    let login_window = egui::Window::new("Main menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .resizable(false)
        .collapsible(false);

    login_window.show(contexts.ctx_mut()?, |ui| {
        if ui.big_button("Play").clicked() {
            debug!("Play button clicked");
            scene_state.set(YvScene::Game);
        }
    });

    Ok(())
}

#[derive(Resource, Default)]
pub(super) struct IngameDebugData {
    add_actor_x: f32,
    add_actor_y: f32,
    add_actor_actor_type: ActorType,

    bulk_add_terrain_start_x: i32,
    bulk_add_terrain_start_y: i32,
    bulk_add_terrain_end_x: i32,
    bulk_add_terrain_end_y: i32,
    add_terrain_chunk_x: i32,
    add_terrain_chunk_y: i32,
    add_terrain_chunk_chunk_type: ChunkType,
    add_terrain_chunk_biome: Biome,
}

/// Ingame debug window.
#[allow(clippy::too_many_arguments)]
fn ingame_debug_window(
    mut contexts: EguiContexts,
    mut add_terrain: EventWriter<AddTerrainChunk>,
    mut generate_terrain: EventWriter<ProcedurallyGenerateTerrain>,
    mut ingame_debug_data: ResMut<IngameDebugData>,
    mut free_camera_state: ResMut<FreeCameraState>,
    mut add_actor: EventWriter<SpawnActor>,
    camera_mode: Res<State<CameraMode>>,
    next_camera_mode: ResMut<NextState<CameraMode>>,
) -> Result {
    let ingame_debug_window = egui::Window::new("Debug")
        .resizable(false)
        .collapsible(true);

    ingame_debug_window.show(contexts.ctx_mut()?, |ui| {
        ui.actor_input_section(&mut ingame_debug_data, &mut add_actor);
        ui.separator();
        ui.camera_section(&mut free_camera_state, camera_mode, next_camera_mode);
        ui.separator();
        ui.terrain_chunk_input_section(
            &mut ingame_debug_data,
            &mut add_terrain,
            &mut generate_terrain,
        );
    });

    Ok(())
}
