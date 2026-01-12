use bevy::prelude::*;

use crate::screens::Screen;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Loadingscreen), (spawn_loading_screen));
    }
}

const DESPAWN_MARKER: DespawnOnExit<Screen> = DespawnOnExit(Screen::Loadingscreen);

#[derive(Component)]
#[require(Camera2d)]
pub struct LoadingScreenCamera;

fn spawn_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((DESPAWN_MARKER, LoadingScreenCamera));
    commands.spawn((DESPAWN_MARKER,
        Node {
            margin: UiRect::all(Val::Auto),
            //this needs to be tuned later
            width: Val::Percent(70.),
  
            ..Default::default()
        },
        ImageNode {
        image: asset_server.load("bevy_logo_dark.png"),
        ..Default::default()
    }));
}
