use bevy::prelude::*;

use crate::screens::Screen;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::TitleScreen), (spawn_title_screen))
            .add_systems(
                Update,
                next_screen_on_click.run_if(in_state(Screen::TitleScreen)),
            );
    }
}

use crate::ui::title_screen;

#[derive(Component)]
#[require(Camera2d)]
pub struct TitleScreenCamera;

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
            (
                Node {
                    height: px(40),
                    margin: UiRect::bottom(px(20)),
                    ..Default::default()
                },
                Text::new("Title")
            ),
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
            title_screen::button(
                "Settings",
                Node {
                    margin: UiRect::all(px(7.5)),

                    width: px(200),
                    height: px(35),
                    align_self: AlignSelf::Start,
                    ..default()
                }
            ),
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
        ],
    ));
}

fn next_screen_on_click(
    mut interaction_query: Query<&Interaction>,
    mut next_state: ResMut<NextState<Screen>>,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            next_state.set(todo!());
        }
    }
}
