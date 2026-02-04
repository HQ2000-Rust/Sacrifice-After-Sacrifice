mod splash;
mod title;
mod util;
mod level_selection;

use bevy::prelude::*;

#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq, States)]
pub enum Screen {
    #[default]
    SplashScreen,
    TitleScreen,
    LevelSelectionScreen,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<Screen>()
            .add_plugins((splash::SplashScreenPlugin, title::TitleScreenPlugin));
    }
}
