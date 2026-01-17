use crate::screens::Screen;
use bevy::prelude::*;

//"inspired" by the bevy `in_state` helper
// kinda dumb, but maybe it actually will be useful
pub fn set_screen(next_screen: Screen) -> impl FnMut(ResMut<'_, NextState<Screen>>) + Clone {
    move |mut next_screen_state: ResMut<NextState<Screen>>| {
        next_screen_state.set(next_screen);
    }
}
