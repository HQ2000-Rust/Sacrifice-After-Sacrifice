use bevy::prelude::*;

use crate::screens::{Screen, util::LevelId};

pub struct LevelSelectionPlugin;

impl Plugin for LevelSelectionPlugin {
    fn build(&self, app: &mut App) {
        app //.add_systems(OnEnter(Screen::LevelSelectionScreen), setup)
            .add_plugins((ScrollbarPlugin, InputDispatchPlugin, TabNavigationPlugin))
            .insert_resource(UiScale(1.25))
            .add_systems(
                OnEnter(Screen::LevelSelectionScreen),
                setup_view_root.run_if(in_state(Screen::LevelSelectionScreen)),
            )
            .add_systems(
                Update,
                (
                update_scrollbar_thumb.run_if(in_state(Screen::LevelSelectionScreen)),
                    handle_button_presses.run_if(in_state(Screen::LevelSelectionScreen)),
                ),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::LevelSelectionScreen),
        Node {
            height: percent(100),
            width: percent(100),
            ..default()
        },
    ));
}

fn start_level(level: Option<LevelId>, next_screen: &mut ResMut<NextState<Screen>>) {
    if let Some(level) = level {
        next_screen.set(Screen::LevelScreen(level));
    }
}

fn handle_button_presses(
    q_buttons: Query<(&LevelButton, &Interaction), Changed<Interaction>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for (button, interaction) in q_buttons.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                LevelButton::Level1 => start_level(LevelId::new(1), &mut next_screen),
            }
        }
    }
}

#[derive(Component)]
enum LevelButton {
    Level1,
}

use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    input_focus::{
        InputDispatchPlugin,
        tab_navigation::{TabGroup, TabNavigationPlugin},
    },
    picking::hover::Hovered,
    ui_widgets::{
        ControlOrientation, CoreScrollbarDragState, CoreScrollbarThumb, Scrollbar, ScrollbarPlugin,
    },
};

fn setup_view_root(mut commands: Commands) {
    let camera = commands.spawn((Camera::default(), Camera2d)).id();

    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
            right: px(0),
            bottom: px(0),
            padding: UiRect::all(px(3)),
            row_gap: px(6),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        UiTargetCamera(camera),
        TabGroup::default(),
        Children::spawn((Spawn(Text::new("Scrolling")), Spawn(scroll_area_demo()))),
    ));
}

/// Create a scrolling area.
///
/// The "scroll area" is a container that can be scrolled. It has a nested structure which is
/// three levels deep:
/// - The outermost node is a grid that contains the scroll area and the scrollbars.
/// - The scroll area is a flex container that contains the scrollable content. This
///   is the element that has the `overflow: scroll` property.
/// - The scrollable content consists of the elements actually displayed in the scrolling area.
fn scroll_area_demo() -> impl Bundle {
    (
        // Frame element which contains the scroll area and scrollbars.
        Node {
            display: Display::Grid,
            width: px(200),
            height: px(150),
            grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.)],
            grid_template_rows: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
            row_gap: px(2),
            column_gap: px(2),
            ..default()
        },
        Children::spawn((SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            // The actual scrolling area.
            // Note that we're using `SpawnWith` here because we need to get the entity id of the
            // scroll area in order to set the target of the scrollbars.
            let scroll_area_id = parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        padding: UiRect::all(px(4)),
                        overflow: Overflow::scroll(),
                        ..default()
                    },
                    BackgroundColor(colors::GRAY1.into()),
                    ScrollPosition(Vec2::new(0.0, 10.0)),
                    Children::spawn(
                        // The actual content of the scrolling area
                        Spawn((LevelButton::Level1, text_row("Level 1"))),
                    ),
                ))
                .id();

            // Vertical scrollbar
            // parent.spawn((
            //     Node {
            //         min_width: px(8),
            //         grid_row: GridPlacement::start(1),
            //         grid_column: GridPlacement::start(2),
            //         ..default()
            //     },
            //     Scrollbar {
            //         orientation: ControlOrientation::Vertical,
            //         target: scroll_area_id,
            //         min_thumb_length: 8.0,
            //     },
            //     Children::spawn(Spawn((
            //         Node {
            //             position_type: PositionType::Absolute,
            //             border_radius: BorderRadius::all(px(4)),
            //             ..default()
            //         },
            //         Hovered::default(),
            //         BackgroundColor(colors::GRAY2.into()),
            //         CoreScrollbarThumb,
            //     ))),
            // ));

            // Horizontal scrollbar
            parent.spawn((
                Node {
                    min_height: px(8),
                    grid_row: GridPlacement::start(2),
                    grid_column: GridPlacement::start(1),
                    ..default()
                },
                Scrollbar {
                    orientation: ControlOrientation::Horizontal,
                    target: scroll_area_id,
                    min_thumb_length: 8.0,
                },
                Children::spawn(Spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        border_radius: BorderRadius::all(px(4)),
                        ..default()
                    },
                    Hovered::default(),
                    BackgroundColor(colors::GRAY2.into()),
                    CoreScrollbarThumb,
                ))),
            ));
        }),)),
    )
}

/// Create a list row
fn text_row(caption: &str) -> impl Bundle {
    (
        Text::new(caption),
        TextFont {
            font_size: 14.0,
            ..default()
        },
    )
}

// Update the color of the scrollbar thumb.
fn update_scrollbar_thumb(
    mut q_thumb: Query<
        (&mut BackgroundColor, &Hovered, &CoreScrollbarDragState),
        (
            With<CoreScrollbarThumb>,
            Or<(Changed<Hovered>, Changed<CoreScrollbarDragState>)>,
        ),
    >,
) {
    for (mut thumb_bg, Hovered(is_hovering), drag) in q_thumb.iter_mut() {
        let color: Color = if *is_hovering || drag.dragging {
            // If hovering, use a lighter color
            colors::GRAY3
        } else {
            // Default color for the slider
            colors::GRAY2
        }
        .into();

        if thumb_bg.0 != color {
            // Update the color of the thumb
            thumb_bg.0 = color;
        }
    }
}

mod colors {
    use bevy::color::Srgba;

    pub const GRAY1: Srgba = Srgba::new(0.224, 0.224, 0.243, 1.0);
    pub const GRAY2: Srgba = Srgba::new(0.486, 0.486, 0.529, 1.0);
    pub const GRAY3: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);
}
