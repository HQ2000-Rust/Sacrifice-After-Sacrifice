mod loading;
mod title;

use bevy::prelude::*;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug, States)]
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
