mod asset_management;
mod screens;

#[cfg(feature = "dev")]
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;

use crate::screens::ScreensPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            #[cfg(feature = "dev")]
            FpsOverlayPlugin::default(),
        ))
        .add_plugins(ScreensPlugin)
        .run()
}
