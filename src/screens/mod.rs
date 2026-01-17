mod loading;
mod title;
mod util;

use bevy::prelude::*;

#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq, States)]
pub enum Screen {
    #[default]
    Loadingscreen,
    TitleScreen,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<Screen>()
            .add_plugins((loading::LoadingScreenPlugin, title::TitleScreenPlugin));
    }
}
