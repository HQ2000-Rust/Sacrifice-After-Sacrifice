mod level_selection;
mod splash;
mod title;
mod util;
mod level;

use bevy::prelude::*;

use crate::screens::util::LevelId;

#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq, States)]
pub enum Screen {
    #[default]
    SplashScreen,
    TitleScreen,
    LevelSelectionScreen,
    LevelScreen(LevelId),
    SettingsScreen,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<Screen>().add_plugins((
            splash::SplashScreenPlugin,
            title::TitleScreenPlugin,
            level_selection::LevelSelectionPlugin,
            level::LevelScreensPlugin,
        ));
    }
}
