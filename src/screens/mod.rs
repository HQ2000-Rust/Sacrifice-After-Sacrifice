mod loading;
mod title;

use bevy::prelude::*;

#[derive(Default, Debug, Hash, Clone, PartialEq, Eq, States)]
pub enum Screen {
    #[default]
    Loadingscreen,
    TitleScreen,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .init_state::<Screen>()
            .add_plugins((
                loading::LoadingScreenPlugin,
                title::TitleScreenPlugin,
                
            ));
            
    }
}