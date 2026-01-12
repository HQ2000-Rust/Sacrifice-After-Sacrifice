mod loading_screen;
mod title_screen;
mod screens;

use bevy::prelude::*;

use loading_screen::loading_screen;

use crate::screens::ScreenPlugin;


fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            ScreenPlugin
        ))
        .add_systems(Startup, loading_screen)
        .run()
}
