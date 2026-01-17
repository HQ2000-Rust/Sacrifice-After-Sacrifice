use bevy::prelude::*;

use crate::screens::{Screen, loading::InitialLoadingFinished};

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::TitleScreen), (spawn_title_screen))
            .add_observer(
                |_trigger: On<InitialLoadingFinished>, mut screen: ResMut<NextState<Screen>>| {
                    screen.set(Screen::TitleScreen);
                },
            );
    }
}

fn spawn_title_screen(mut commands: Commands) {
    commands.spawn((DespawnOnExit(Screen::TitleScreen)));
}
