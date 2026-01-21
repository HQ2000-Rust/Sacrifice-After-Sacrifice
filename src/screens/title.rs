use bevy::prelude::*;

use crate::screens::Screen;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::TitleScreen), (spawn_title_screen));
    }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct TitleScreenCamera;

fn spawn_title_screen(mut commands: Commands) {
    commands.spawn((DespawnOnExit(Screen::TitleScreen), TitleScreenCamera));
    commands.spawn((
        DespawnOnExit(Screen::TitleScreen),
        Node::DEFAULT,
        Text2d::new("Title"),
    ));
}
