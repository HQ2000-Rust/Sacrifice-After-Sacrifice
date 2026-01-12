use bevy::{ecs::component, prelude::*};

use crate::{asset_management::loading_screen::BevyLogo, screens::Screen};

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Loadingscreen), (spawn_loading_screen));
    }
}

const DESPAWN_MARKER: DespawnOnExit<Screen> =DespawnOnExit(Screen::Loadingscreen);

#[derive(Component)]
#[require(
    Camera2d
)]
pub struct LoadingScreenCamera;

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((DESPAWN_MARKER,
        LoadingScreenCamera));
    commands.spawn((
        DESPAWN_MARKER,
        
    ));
}
