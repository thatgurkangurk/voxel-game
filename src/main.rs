use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use common::AppState;
use plugins::{camera::CameraPlugin, hud::HudPlugin, keyboard::KeyboardPlugin, world::WorldPlugin};

mod common;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            filter:
                "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info".to_string(),
        }))
        .add_state::<AppState>()
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(KeyboardPlugin)
        .add_plugins(HudPlugin)
        .run();
}
