mod splash;
mod title;
mod util;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq, States)]
pub enum Screen {
    #[default]
    SplashScreen,
    TitleScreen,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<Screen>()
            .add_plugins((splash::SplashScreenPlugin, title::TitleScreenPlugin));
    }
}
