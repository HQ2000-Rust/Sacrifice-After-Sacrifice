use bevy::prelude::*;

pub mod button {
    use bevy::prelude::*;
    pub const STANDARD_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.3);
    pub const HOVERED_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.55);
    pub const PRESSED_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.8);
}
pub fn button(text: impl Into<String>, button_node: Node) -> impl Bundle {
    (
        Button,
        button_node,
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

pub fn title_text(text: impl Into<String>, node: Node) -> impl Bundle {
    (node, Text::new(text))
}
