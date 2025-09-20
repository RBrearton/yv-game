use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};
use yv_core::terrain::{AddTerrainChunk, Biome, ChunkType, TerrainIndex};

use crate::scenes::YvScene;

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
    );
}
impl UiExtensions for egui::Ui {
    fn big_button(&mut self, text: &str) -> egui::Response {
        self.add_sized(
            [self.available_width() * 0.8, 36.0],
            egui::Button::new(egui::RichText::new(text).size(16.0).strong()),
        )
    }

    fn terrain_chunk_input_section(
        &mut self,
        ingame_debug_data: &mut IngameDebugData,
        add_terrain: &mut EventWriter<AddTerrainChunk>,
    ) {
        // Terrain chunk debugging.
        self.heading("Terrain chunk");
        self.horizontal(|ui| {
            ui.label("X index (enter an integer):");
            ui.text_edit_singleline(&mut ingame_debug_data.add_terrain_chunk_x);
        });
        self.horizontal(|ui| {
            ui.label("Y index (enter an integer):");
            ui.text_edit_singleline(&mut ingame_debug_data.add_terrain_chunk_y);
        });
        self.horizontal(|ui| {
            ui.label("Chunk type:");
            ui.selectable_value(
                &mut ingame_debug_data.add_terrain_chunk_chunk_type,
                ChunkType::Grass,
                "Grass",
            );
            ui.selectable_value(
                &mut ingame_debug_data.add_terrain_chunk_chunk_type,
                ChunkType::Water,
                "Water",
            );
        });
        self.horizontal(|ui| {
            ui.label("Biome:");
            ui.selectable_value(
                &mut ingame_debug_data.add_terrain_chunk_biome,
                Biome::Meadow,
                "Meadow",
            );
            ui.selectable_value(
                &mut ingame_debug_data.add_terrain_chunk_biome,
                Biome::Snow,
                "Snow",
            );
        });
        if self.button("Add terrain chunk").clicked() {
            add_terrain.write(AddTerrainChunk::new(
                TerrainIndex::new(
                    ingame_debug_data.add_terrain_chunk_x.parse().unwrap(),
                    ingame_debug_data.add_terrain_chunk_y.parse().unwrap(),
                ),
                ingame_debug_data.add_terrain_chunk_chunk_type,
                ingame_debug_data.add_terrain_chunk_biome,
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

#[derive(Resource)]
pub(super) struct IngameDebugData {
    add_terrain_chunk_x: String,
    add_terrain_chunk_y: String,
    add_terrain_chunk_chunk_type: ChunkType,
    add_terrain_chunk_biome: Biome,
}
impl Default for IngameDebugData {
    fn default() -> Self {
        Self {
            add_terrain_chunk_x: String::new(),
            add_terrain_chunk_y: String::new(),
            add_terrain_chunk_chunk_type: ChunkType::Grass,
            add_terrain_chunk_biome: Biome::Meadow,
        }
    }
}

/// Ingame debug window.
fn ingame_debug_window(
    mut contexts: EguiContexts,
    mut add_terrain: EventWriter<AddTerrainChunk>,
    mut ingame_debug_data: ResMut<IngameDebugData>,
) -> Result {
    let ingame_debug_window = egui::Window::new("Debug")
        .resizable(false)
        .collapsible(true);

    ingame_debug_window.show(contexts.ctx_mut()?, |ui| {
        ui.terrain_chunk_input_section(&mut ingame_debug_data, &mut add_terrain);
    });

    Ok(())
}
