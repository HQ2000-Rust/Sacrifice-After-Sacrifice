use crate::ui::{buttons::main_button, text::main_title};
use crate::{GameState, level, main_menu, setup as general_setup};
use bevy::{color::palettes::basic::*, prelude::*, winit::WinitSettings};
use crate::util::close_app;
use colored::Colorize;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup);
        app.add_systems(Update, main_menu_loop.run_if(is_in_main_menu));
        app.add_systems(
            Update,
            close_app
            .run_if(is_in_main_menu)
                .run_if(got_close_window_command)
        );
    }
}

#[derive(Event)]
pub struct Init;

#[derive(Component)]
pub struct PlayButton;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //info!("{}", "Main menu setup".bright_green().bold());
    info!("AssetServer: {:?}", &asset_server);
    commands.spawn((StateScoped(GameState::MainMenu), Camera2d));
    commands.spawn((
        PlayButton,
        StateScoped(GameState::MainMenu),
        main_button(&asset_server, "Play".to_string()),
    ));
    commands.spawn((
        StateScoped(GameState::MainMenu),
        main_title(&asset_server, "Sacrifice After Sacrifice"),
    ));
    //info!("{}", "Main menu setup completed".bright_green().bold());
}

pub fn is_in_main_menu(game_state: Res<State<GameState>>) -> bool {
    game_state.get() == &GameState::MainMenu
}

//partially copied from an official example, changing this later anyways
pub fn main_menu_loop(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, mut border_color, children) in
        interaction_query.iter_mut()
    {
        match interaction {
            &Interaction::None => {
                *background_color = BackgroundColor(Color::Srgba(RED).with_alpha(0.6));
                *border_color = BorderColor(Color::Srgba(BLACK).with_alpha(0.6));
            }
            &Interaction::Hovered => {
                *background_color = BackgroundColor(Color::Srgba(RED));
                *border_color = BorderColor(Color::Srgba(BLACK));
            }
            &Interaction::Pressed => {
                //*background_color=BackgroundColor(Color::Srgba(RED));
                //*border_color=BorderColor(Color::Srgba(BLACK));
                info!("clicked!");
                game_state.set(GameState::Level);
            }
        }
    }
}

pub fn got_close_window_command(key_code: Res<ButtonInput<KeyCode>>) -> bool {
    if key_code.any_just_pressed([KeyCode::Escape, KeyCode::Abort]) {
        return true;
    }
    false
}
