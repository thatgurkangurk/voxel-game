use bevy::prelude::*;

use crate::common::AppState;
pub struct KeyboardPlugin;

#[derive(Event)]
pub struct ResetMapEvent;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetMapEvent>()
            .add_systems(Update, start_screen.run_if(in_state(AppState::Setup)))
            .add_systems(Update, reset.run_if(in_state(AppState::Finished)));
    }
}

fn start_screen(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::Return) {
        next_state.set(AppState::Build);
    }
}

fn reset(mut events: EventWriter<ResetMapEvent>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::R) {
        events.send(ResetMapEvent);
    }
}
