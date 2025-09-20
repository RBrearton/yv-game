use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use crate::scenes::YvScene;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.add_systems(
            EguiPrimaryContextPass,
            login_window.run_if(in_state(YvScene::MainMenu)),
        );
    }
}

/// Extension trait for `egui::Ui` to add some custom methods.
pub(super) trait UiExtensions {
    fn big_button(&mut self, text: &str) -> egui::Response;
}

impl UiExtensions for egui::Ui {
    fn big_button(&mut self, text: &str) -> egui::Response {
        self.add_sized(
            [self.available_width() * 0.8, 36.0],
            egui::Button::new(egui::RichText::new(text).size(16.0).strong()),
        )
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
