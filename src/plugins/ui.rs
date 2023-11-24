use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, RichText},
    EguiContexts,
};

use crate::{common::AppState, events::ResetMapEvent};

#[derive(Resource)]
pub struct UIWorldState {
    pub width: usize,
    pub height: usize,
}

impl Default for UIWorldState {
    fn default() -> Self {
        UIWorldState {
            width: 100,
            height: 100,
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UIWorldState>()
            .add_systems(Update, world_settings);
    }
}

fn world_settings(
    mut ui_world_state: ResMut<UIWorldState>,
    mut contexts: EguiContexts,
    mut events: EventWriter<ResetMapEvent>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new(
        RichText::new("World Settings")
            .heading()
            .color(Color32::from_rgb(255, 255, 255)),
    )
    .show(ctx, |ui| {
        ui.add(
            egui::Slider::new(&mut ui_world_state.width, 0..=1000)
                .text(RichText::new("Width").color(Color32::from_rgb(255, 255, 255))),
        );

        ui.add(
            egui::Slider::new(&mut ui_world_state.height, 0..=1000)
                .text(RichText::new("Height").color(Color32::from_rgb(255, 255, 255))),
        );

        if ui
            .button(RichText::new("Generate").color(Color32::from_rgb(255, 255, 255)))
            .clicked()
        {
            if state.get() != &AppState::Finished {
                next_state.set(AppState::Build);
            }
            events.send(ResetMapEvent)
        }
    });
}
