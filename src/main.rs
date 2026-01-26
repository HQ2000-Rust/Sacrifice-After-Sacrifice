mod asset_management;
mod screens;
pub mod ui;

#[cfg(feature = "dev")]
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;

use crate::screens::ScreensPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        #[cfg(feature = "dev")]
        FpsOverlayPlugin::default(),
    ))
    .add_plugins(ScreensPlugin);

    #[cfg(feature = "ui_debug")]
    app.insert_resource(UiDebugOptions {
        enabled: true,
        // `UiDebugOptions` has a few new options, but for now we'll leave the defaults.
        ..default()
    });

    app.run()
}
