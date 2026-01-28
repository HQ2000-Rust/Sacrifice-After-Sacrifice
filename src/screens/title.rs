use bevy::prelude::*;

use crate::screens::Screen;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::TitleScreen), spawn_title_screen)
            .add_systems(
                Update,
                (button_cosmetic_effects,
                //next_screen_on_click
                ).run_if(in_state(Screen::TitleScreen)),
            );
    }
}

use crate::ui::title_screen;

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

fn spawn_title_screen(mut commands: Commands) {
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
            title_screen::title_text(
                "Sacrifice After Sacrifice",
                Node {
                    height: px(40),
                    margin: UiRect::all(px(7.5)),
                    ..Default::default()
                }
            ),
            (
                TitleScreenButton::Start,
                title_screen::button(
                    "Start",
                    Node {
                        margin: UiRect::all(px(7.5)),

                        width: px(200),
                        height: px(35),
                        align_self: AlignSelf::Start,
                        ..default()
                    }
                ),
            ),
            (
                TitleScreenButton::Settings,
                title_screen::button(
                    "Settings",
                    Node {
                        margin: UiRect::all(px(7.5)),

                        width: px(200),
                        height: px(35),
                        align_self: AlignSelf::Start,
                        ..default()
                    }
                )
            ),
            (
                TitleScreenButton::About,
                title_screen::button(
                    "About",
                    Node {
                        margin: UiRect::all(px(7.5)),

                        width: px(200),
                        height: px(35),
                        align_self: AlignSelf::Start,
                        ..default()
                    }
                )
            ),
            (
                TitleScreenButton::Quit,
                title_screen::button(
                    "Quit",
                    Node {
                        margin: UiRect::all(px(7.5)),

                        width: px(200),
                        height: px(35),
                        align_self: AlignSelf::Start,
                        ..default()
                    }
                ),
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
                *bg_color = BackgroundColor(title_screen::button::HOVERED_COLOR);
            }
            Interaction::Pressed => {
                *bg_color = BackgroundColor(title_screen::button::PRESSED_COLOR);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(title_screen::button::STANDARD_COLOR);
            }
        }
    }
}
