use bevy::{asset::meta::Settings, audio::Volume, prelude::*};

use crate::{screens::Screen, ui::title_screen};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::SettingsScreen), setup)
            .add_systems(OnExit(Screen::SettingsScreen), cleanup);
    }
}
#[derive(Resource, Default)]
pub struct SettingsCache(pub GameSettings);

#[derive(Resource, Default)]
pub struct GameSettings {
    pub volume: Volume,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("entering settings screen");
    commands.init_resource::<SettingsCache>();
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),

            ..default()
        },
        children![(title_screen::button("Apply", &asset_server))],
    ));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<SettingsCache>();
}
