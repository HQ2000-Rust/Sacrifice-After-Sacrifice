use bevy::prelude::*;

pub fn title_screen_button(text: impl Into<String>, button_node: Node) -> impl Bundle {
    (children![(
        Button,
        button_node,
        children![(Text::new(text),)]
    )],)
}
