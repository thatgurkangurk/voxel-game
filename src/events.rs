use bevy::prelude::*;

#[derive(Event)]
pub struct ResetMapEvent;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ResetMapEvent>();
    }
}
