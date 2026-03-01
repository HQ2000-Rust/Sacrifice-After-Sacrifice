use bevy::{prelude::*, state::commands};

use crate::screens::{Screen, util::set_screen};

pub fn level_select(mut next_screen_state: ResMut<NextState<Screen>>) {
    next_screen_state.set(Screen::LevelSelectionScreen);
}

pub fn about() {
    todo!()
}

pub fn settings() {
    todo!()
}

pub fn quit(mut exit_writer: MessageWriter<AppExit>) {
    exit_writer.write(AppExit::Success);
}
