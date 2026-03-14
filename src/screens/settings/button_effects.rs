use crate::screens::settings::{GameSettings, GameSettingsCache};
use bevy::prelude::*;

//"Dumb" version (TODO)
pub fn apply_settings(mut settings: ResMut<GameSettings>, settings_cache: Res<GameSettingsCache>) {
    *settings=settings_cache.0.clone();
}