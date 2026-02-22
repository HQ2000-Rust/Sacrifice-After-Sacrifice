use bevy::{prelude::*, state::commands};

use crate::screens::{Screen, util::set_screen};

pub fn level_select(mut commands: Commands) {
    commands.run_system_cached(set_screen(Screen::LevelSelectionScreen));
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
