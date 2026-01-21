mod asset_management;
mod screens;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::screens::{Screen, ScreensPlugin};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreensPlugin)
        .run()
}
