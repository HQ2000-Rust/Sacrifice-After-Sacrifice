mod main_menu;
mod level;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(State::new(GameState::MainMenu))
        .run();
}

#[derive(States)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    //maybe unused
    #[default]
    MainMenu,
    Settings,
    Level(level::Level),
    Paused,
}
