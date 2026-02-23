//TODO: screen state switching, about, settings

use bevy::prelude::*;

mod button_effects;

use crate::screens::Screen;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::TitleScreen), setup)
            .add_systems(
                Update,
                (button_cosmetic_effects, handle_button_presses)
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

//put them together later

fn handle_button_presses(
    mut interaction_query: Query<(&Interaction, &TitleScreenButton), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            //system_cached_with if we want to supply e.g. the interaction state (-> first param With<> needed)
            match button {
                TitleScreenButton::Start => {
                    commands.run_system_cached(button_effects::level_select)
                }
                TitleScreenButton::Settings => commands.run_system_cached(button_effects::settings),
                TitleScreenButton::About => commands.run_system_cached(button_effects::about),
                TitleScreenButton::Quit => commands.run_system_cached(button_effects::quit),
            };
        }
    }
}

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
