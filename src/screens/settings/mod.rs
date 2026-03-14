use bevy::{asset::meta::Settings, audio::Volume, prelude::*};

use crate::screens::Screen;

use crate::ui::default as ui;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::SettingsScreen), setup)
            .add_systems(OnExit(Screen::SettingsScreen), cleanup);
    }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct SettingsScreenCamera;

#[derive(Component)]
pub enum SettingsScreenButton {
    Apply,
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
    commands.spawn((DespawnOnExit(Screen::SettingsScreen), SettingsScreenCamera));
    commands.spawn((
        DespawnOnExit(Screen::SettingsScreen),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(
            ui::default_button_node(),
            ui::button("Apply", &asset_server)
        )],
    ));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<SettingsCache>();
}
