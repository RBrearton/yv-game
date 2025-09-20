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

fn login_window(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Login").show(contexts.ctx_mut()?, |ui| {
        ui.label("Login");
    });
    Ok(())
}
