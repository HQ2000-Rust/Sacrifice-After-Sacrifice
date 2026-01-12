mod asset_management;
mod screens;

use bevy::prelude::*;

use crate::screens::ScreensPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ScreensPlugin))
        .run()
}
