use std::ops::{RangeBounds, RangeInclusive};

use crate::screens::Screen;
use bevy::prelude::*;

//"inspired" by the bevy `in_state` helper
// kinda dumb, but maybe it actually will be useful
pub fn set_screen(next_screen: Screen) -> impl FnMut(ResMut<'_, NextState<Screen>>) + Clone {
    move |mut next_screen_state: ResMut<NextState<Screen>>| {
        next_screen_state.set(next_screen);
    }
}

const LEVEL_RANGE: RangeInclusive<u8>= 1..=1;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LevelId(u8);

impl LevelId {
    pub fn new(id: u8) -> Option<LevelId> {
        if LEVEL_RANGE.contains(&id) {
            Some(LevelId(id))
        }
        else {
            None
        }
    }
    
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for LevelId {
    type Error=();
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        LevelId::new(value).ok_or(())
    }
}

impl Into<u8> for LevelId {
    fn into(self) -> u8 {
        self.0
    }
}
