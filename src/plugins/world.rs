use bevy::prelude::*;
use bevy::render::color::Color;
use noise::{utils::*, BasicMulti, Perlin};
use rand::{thread_rng, Rng};

use crate::{common::AppState, events::ResetMapEvent};

use super::ui::UIWorldState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Build), generate_world)
            .add_systems(OnExit(AppState::Finished), cleanup)
            .add_systems(Update, reset_listener.run_if(in_state(AppState::Finished)));
    }
}

fn generate_noise_map(width: usize, height: usize) -> NoiseMap {
    let mut rng = thread_rng();
    let seed: u32 = rng.gen();

    let basicmulti = BasicMulti::<Perlin>::new(seed);

    PlaneMapBuilder::<_, 2>::new(&basicmulti)
        .set_size(width, height)
        .build()
}

#[derive(Resource, Deref)]
struct Root(Entity);

fn get_colour(val: f64) -> Color {
    let color_result = match val.abs() {
        v if v < 0.1 => Color::hex("#0a7e0a"),
        v if v < 0.2 => Color::hex("#0da50d"),
        v if v < 0.3 => Color::hex("#10cb10"),
        v if v < 0.4 => Color::hex("#18ed18"),
        v if v < 0.5 => Color::hex("#3ff03f"),
        v if v < 0.6 => Color::hex("#65f365"),
        v if v < 0.7 => Color::hex("#8cf68c"),
        v if v < 0.8 => Color::hex("#b2f9b2"),
        v if v < 0.9 => Color::hex("#d9fcd9"),
        v if v <= 1.0 => Color::hex("#ffffff"),
        _ => panic!("unexpected value"),
    };
    color_result.expect("Getting color from HEX error")
}

fn generate_world(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    ui_world_state: Res<UIWorldState>,
) {
    let map = generate_noise_map(ui_world_state.width, ui_world_state.height);
    let (grid_width, grid_height) = map.size();
    debug!("map size: {}x{}", grid_width, grid_height);

    let tile_size = 32_f32;

    let start_x = -(grid_width as f32) * tile_size / 2.0;
    let start_y = -(grid_height as f32) * tile_size / 2.0;

    let root = commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            for col_x in 0..grid_width {
                for col_y in 0..grid_height {
                    let val = map.get_value(col_x, col_y);

                    let x = start_x + col_x as f32 * tile_size;
                    let y = start_y + col_y as f32 * tile_size;

                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: get_colour(val),
                            custom_size: Some(Vec2::new(tile_size, tile_size)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                        ..default()
                    });
                }
            }
        })
        .id();

    commands.insert_resource(Root(root));

    next_state.set(AppState::Finished);
}

fn cleanup(mut commands: Commands, root: Res<Root>) {
    commands.entity(**root).despawn_recursive();
}

fn reset_listener(
    mut events: EventReader<ResetMapEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in events.read() {
        next_state.set(AppState::Build);
    }
}
