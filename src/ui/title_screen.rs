use bevy::prelude::*;

pub fn button(text: impl Into<String>, button_node: Node) -> impl Bundle {
    (
        Button,
        button_node,
        children![(Text::new(text),)]
    )
}
