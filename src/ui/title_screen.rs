use bevy::prelude::*;

pub mod button {
    use bevy::prelude::*;
    pub const STANDARD_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.3);
    pub const HOVERED_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.55);
    pub const PRESSED_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.8);
}

//no constant since (Default::)default() isn't const
pub fn default_button_node() -> Node {
    Node {
        margin: UiRect::all(px(7.5)),

        width: px(200),
        height: px(35),
        align_self: AlignSelf::Start,
        ..default()
    }
}

//no constant since (Default::)default() isn't const
pub fn title_node() -> Node {
    Node {
        height: px(40),
        margin: UiRect::all(px(7.5)),
        ..default()
    }
}

pub fn button(text: impl Into<String>) -> impl Bundle {
    (
        Button,
        children![(
            Text::new(text),
            Node {
                left: px(7.5),
                top: px(7.5),
                bottom: px(7.5),
                ..default()
            },
        )],
        BackgroundColor(button::STANDARD_COLOR),
    )
}

pub fn title_text(text: impl Into<String>) -> impl Bundle {
    (Text::new(text))
}
