mod asset_management;
mod screens;

use bevy::prelude::*;

use crate::screens::{Screen, ScreensPlugin};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ScreensPlugin))
        .add_systems(detect_screen_change)
        .run()
}

fn detect_screen_change(now: Res<State<Screen>>, next: Res<NextState<Screen>>) {
    match *next {
        NextState::Pending()
        NextState::Unchanged => {}
    }
}
