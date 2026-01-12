use bevy::{app::Plugin, ecs::resource::Resource};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,Resource,Debug)]
pub enum CurrentScreen {
    #[default]
    NoScreen,
    Loadingscreen,
    TitleScreen,
}

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(CurrentScreen::NoScreen);
    }
}