use bevy::prelude::*;

use crate::screens::Screen;

pub struct LoadingScreenPlugin;


impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(Screen::Loadingscreen), (
                spawn_loading_screen
            ))
        ;
    }   
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::Loadingscreen),
        
        Node {
            ..Default::default()
        }
    ));
}