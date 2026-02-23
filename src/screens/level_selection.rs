use bevy::prelude::*;

use crate::screens::Screen;

pub struct LevelSelectionPlugin;

impl Plugin for LevelSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::LevelSelectionScreen), setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::LevelSelectionScreen),
        Node {
            height: percent(100),
            width: percent(100),
            ..default()
        },
        children![
            
        ],
    ));
}
