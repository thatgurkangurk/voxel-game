use bevy::prelude::*;

use crate::common::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Setup), setup_camera);
        app.add_systems(Update, camera_movement.run_if(in_state(AppState::Finished)));
    }
}

// orthographic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut ortho)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }

        if input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }

        if input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        if input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        if input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let translation = &mut transform.translation;
        *translation += time.delta_seconds() * 500.0 * direction;
    }
}
