use bevy::prelude::*;

use crate::common::AppState;

#[derive(Component)]
struct StatusText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, text_update_system)
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        (TextBundle::from_section(
            "status is loading",
            TextStyle {
                font: asset_server.load("fonts/SpaceGrotesk.ttf"),
                font_size: 45.0,
                color: Color::rgb(255., 0., 0.),
                ..default()
            },
        ))
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(5.0),
            right: Val::Percent(10.0),
            ..default()
        }),
        StatusText,
    ));
}

fn text_update_system(mut query: Query<&mut Text, With<StatusText>>, state: Res<State<AppState>>) {
    for mut text in &mut query {
        let status = state.get();
        match status {
            AppState::Setup => text.sections[0].value = "press enter to generate".to_string(),
            AppState::Build => text.sections[0].value = "building".to_string(),
            AppState::Finished => text.sections[0].value = "press r to regenerate".to_string(),
        }
    }
}
