use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use plugins::camera::CameraPlugin;

mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            filter:
                "wgpu=error,naga=error,bevy_render=error,bevy_app=info,bevy_ecs=info".to_string(),
        }))
        .add_plugins(CameraPlugin)
        .run();
}
