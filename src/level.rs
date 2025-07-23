use crate::{GameState, level};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Level), setup);
        app.add_systems(Update, level_loop.run_if(is_in_level));
        app.init_state::<Level>();
        app.init_state::<Paused>();
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd, States)]
#[states(scoped_entities)]
pub enum Level {
    #[default]
    None,
    L1,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd, States)]
#[states(scoped_entities)]
pub struct Paused(bool);

pub fn setup(mut commands: Commands) {}

pub fn is_in_level(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::Level
}

fn level_loop() {}
