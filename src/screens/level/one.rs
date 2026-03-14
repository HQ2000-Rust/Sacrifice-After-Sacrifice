use bevy::prelude::*;

use crate::screens::{Screen, util::LevelId};

pub struct LevelOnePlugin;

impl Plugin for LevelOnePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(Screen::LevelScreen(LevelId::new(1).expect(
                "there exists a level. if a level exists, there exists a first level",
            ))),
            setup,
        ) // TODO
        .add_systems(
            Update,
            update.run_if(in_state(Screen::LevelScreen(
                LevelId::new(1).expect("there exists a level"),
            ))), // TODO
        );
    }
}

fn setup(mut commands: Commands) {}

fn update() {}
