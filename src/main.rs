mod camera;
mod level;
mod main_menu;
mod start_screen;
mod ui;
mod windowing;
mod util;

use bevy::prelude::*;
use bevy::winit::WinitSettings;

use crate::level::{Level, LevelPlugin};
use crate::main_menu::{Init as MainMenuInit, MainMenuPlugin};
use crate::start_screen::StartScreenPlugin;
use crate::windowing::fullscreen;
use colored::Colorize;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GeneralSetupPlugin)
        .add_plugins(StartScreenPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(LevelPlugin)
        .run();
}

struct GeneralSetupPlugin;

impl Plugin for GeneralSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::desktop_app());
        app.init_state::<GameState>();
        app.add_systems(Startup, (fullscreen, setup).chain());
        app.add_event::<MainMenuInit>();
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, States)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    StartScreen,
    MainMenu,
    Settings,
    Level,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut init_event: EventWriter<MainMenuInit>,
) {
    info!("{}", "Setup".bright_white().bold());
    //asset_server.load("")
    info!("{}", "Setup complete".bright_white().bold());
}
