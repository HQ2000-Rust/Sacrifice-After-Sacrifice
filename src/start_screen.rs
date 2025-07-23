use crate::GameState;
use crate::ui::logo::bevy_logo;
use bevy::prelude::*;
use crate::ui::text::loading_screen;

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartScreen), setup);
        app.add_systems(
            Update,
            main_menu_check
                .run_if(resource_changed::<ButtonInput<KeyCode>>)
                .run_if(is_in_start_screen),
        );
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((StateScoped(GameState::StartScreen), Camera2d));
    commands.spawn((
        StateScoped(GameState::StartScreen),
        bevy_logo(&asset_server),
    ));
    commands.spawn((
        StateScoped(GameState::StartScreen),
        loading_screen(&asset_server, "Loading")
        ));
}

pub fn is_in_start_screen(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::StartScreen
}

pub fn main_menu_check(
    mut game_state: ResMut<NextState<GameState>>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
if keycode.just_pressed(KeyCode::Space) {
    game_state.set(GameState::MainMenu);
}
}
