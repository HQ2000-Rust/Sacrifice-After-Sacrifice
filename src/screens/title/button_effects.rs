use bevy::prelude::*;

pub fn level_select() {
    todo!()
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
