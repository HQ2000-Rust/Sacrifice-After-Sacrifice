mod button_effects;

use bevy::{asset::meta::Settings, audio::Volume, prelude::*};

use crate::screens::Screen;

use crate::screens::settings::button_effects::apply_settings;
use crate::ui::default as ui;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::SettingsScreen), setup)
            .add_systems(OnExit(Screen::SettingsScreen), cleanup)
            .add_systems(
                            Update,
                            (ui::button_cosmetic_effects, handle_button_presses)
                                .run_if(in_state(Screen::SettingsScreen)),
                        );
    }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct SettingsScreenCamera;

#[derive(Component)]
pub enum SettingsScreenButton {
    Apply,
}

#[derive(Resource, Default, Clone)]
pub struct GameSettingsCache(pub GameSettings);

#[derive(Resource, Default, Clone)]
pub struct GameSettings {
    pub volume: Volume,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("entering settings screen");

    commands.init_resource::<GameSettingsCache>();
    commands.spawn((DespawnOnExit(Screen::SettingsScreen), SettingsScreenCamera));
    commands.spawn((
        DespawnOnExit(Screen::SettingsScreen),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(
            SettingsScreenButton::Apply,
            ui::default_button_node(),
            ui::button("Apply", &asset_server)
        )],
    ));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<GameSettingsCache>();
}

fn handle_button_presses(
    mut interaction_query: Query<(&Interaction, &SettingsScreenButton), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            //system_cached_with if we want to supply e.g. the interaction state (-> first param With<> needed)
            match button {
                SettingsScreenButton::Apply => {
                    commands.run_system_cached(apply_settings);
                }
            };
        }
    }
}