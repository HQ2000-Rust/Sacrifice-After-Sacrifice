use bevy::prelude::*;

use crate::screens::Screen;

pub fn level_select(mut next_screen_state: ResMut<NextState<Screen>>) {
    next_screen_state.set(Screen::LevelSelectionScreen);
}

pub fn about() {
    todo!()
}

pub fn settings(mut next_screen_state: ResMut<NextState<Screen>>) {
    next_screen_state.set(Screen::SettingsScreen);
}

pub fn quit(mut exit_writer: MessageWriter<AppExit>) {
    exit_writer.write(AppExit::Success);
}
