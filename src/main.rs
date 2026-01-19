mod asset_management;
mod screens;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::screens::{Screen, ScreensPlugin};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ScreensPlugin))
        .add_systems(PreUpdate,detect_screen_change)
        .add_systems(Update, (|mut state:  ResMut<NextState<Screen>>| {
            state.set(Screen::TitleScreen);
            info!("manual change to title screen via space key")
            }).run_if(input_just_pressed(KeyCode::Space)))
        .run()
}

fn detect_screen_change(mut last: Local<Option<Screen>>,now: Res<State<Screen>>) {
    last.and_then(|last| {
        if last!=*now.get() {
            info!("state changed to {:?}", *now)
        }
        Some(last)
    });
    *last= Some(*now.get());
    
}
