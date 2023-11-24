use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_egui::EguiPlugin;
use common::AppState;
use events::EventsPlugin;
use plugins::{camera::CameraPlugin, hud::HudPlugin, ui::UIPlugin, world::WorldPlugin};

mod common;
mod events;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            filter:
                "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info".to_string(),
        }))
        .add_plugins(EventsPlugin)
        .add_plugins(EguiPlugin)
        .add_state::<AppState>()
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(UIPlugin)
        .run();
}
