use bevy::prelude::*;

use crate::screens::Screen;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::TitleScreen), (spawn_title_screen));
    }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct TitleScreenCamera;

fn spawn_title_screen(mut commands: Commands) {
    commands.spawn((DespawnOnExit(Screen::TitleScreen), TitleScreenCamera));
    commands.spawn((
        DespawnOnExit(Screen::TitleScreen),
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![
            (
                Node {
                    margin: UiRect::bottom(Val::Px(20.)),
                    ..Default::default()
                },
                Text::new("Title")
            ),
            (Button, children![(Node::DEFAULT, Text::new("Start"))])
        ],
    ));
}
