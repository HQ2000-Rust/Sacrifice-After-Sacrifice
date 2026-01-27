use bevy::prelude::*;

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
        BackgroundColor(Color::linear_rgba(0., 0., 0., 0.4)),
    )
}

pub fn title_text(text: impl Into<String>, node: Node) -> impl Bundle {
    (node, Text::new(text))
}
