use bevy::prelude::*;

use crate::screens::Screen;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::TitleScreen), spawn_title_screen)
            .add_systems(
                Update,
                (
                    button_cosmetic_effects,
                    //next_screen_on_click
                )
                    .run_if(in_state(Screen::TitleScreen)),
            );
    }
}

use crate::ui::title_screen as ui;

#[derive(Component)]
#[require(Camera2d)]
pub struct TitleScreenCamera;

#[derive(Component)]
pub enum TitleScreenButton {
    Start,
    Settings,
    About,
    Quit,
}

fn spawn_title_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((DespawnOnExit(Screen::TitleScreen), TitleScreenCamera));
    commands.spawn((
        DespawnOnExit(Screen::TitleScreen),
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            height: percent(100),
            width: percent(100),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![
            (
                ui::title_text("Sacrifice After Sacrifice", &asset_server),
                ui::title_node()
            ),
            (
                TitleScreenButton::Start,
                ui::default_button_node(),
                ui::button("Start", &asset_server),
            ),
            (
                TitleScreenButton::Settings,
                ui::default_button_node(),
                ui::button("Settings", &asset_server)
            ),
            (
                TitleScreenButton::About,
                ui::default_button_node(),
                ui::button("About", &asset_server)
            ),
            (
                TitleScreenButton::Quit,
                ui::default_button_node(),
                ui::button("Quit", &asset_server),
            )
        ],
    ));
}

//TODO
/*fn next_screen_on_click(
    mut interaction_query: Query<(&Interaction, &TitleScreenButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    for (interaction, button_type) in interaction_query.iter_mut() {
        /*if button_type == TitleScreenButton::Start && *interaction == Interaction::Pressed {
            next_state.set(todo!());
        }*/
    }
}*/

fn button_cosmetic_effects(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Children),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut bg_color, mut children) in interaction_query {
        match interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(ui::button::HOVERED_COLOR);
            }
            Interaction::Pressed => {
                *bg_color = BackgroundColor(ui::button::PRESSED_COLOR);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(ui::button::STANDARD_COLOR);
            }
        }
    }
}
