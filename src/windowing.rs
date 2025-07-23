use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

//also copied from the docs
pub fn fullscreen(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    // Query returns one window typically.
    for mut window in windows.iter_mut() {
        window.mode =
            WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
    }
}
