use bevy::prelude::*;

use crate::{asset_management::loading_screen::BevyLogo, screens::Screen};

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Loadingscreen), (spawn_loading_screen));
    }
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::Loadingscreen),
        Node {
            ..Default::default()
        },
    ));
}

fn load_assets(mut bevy_logo: ResMut<BevyLogo>, asset_server: Res<AssetServer>) {
    bevy_logo.0=asset_server.load::<Image>("bevy_logo_dark.svg");
}
