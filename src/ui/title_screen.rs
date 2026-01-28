use bevy::prelude::*;

const UI_SCALE: f32=1.;

pub mod text {
    use bevy::prelude::*;
    pub const BUTTON_FONT_PATH: Option<&str>=None;
    pub const BUTTON_FONT_SIZE: f32=30.;

    pub const TITLE_FONT_PATH: Option<&str>=None; 
    pub const TITLE_FONT_SIZE: f32=50.;

}

pub mod button {
    use bevy::prelude::*;
    pub const STANDARD_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.3);
    pub const HOVERED_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.55);
    pub const PRESSED_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.8);
}

//no constant since (Default::)default() isn't const
pub fn default_button_node() -> Node {
    Node {
        margin: UiRect::all(px(UI_SCALE*15.)),

        width: px(UI_SCALE*400.),
        height: px(UI_SCALE*70.),
        align_self: AlignSelf::Start,
        ..default()
    }
}

//no constant since (Default::)default() isn't const
pub fn title_node() -> Node {
    Node {
        height: px(UI_SCALE*80.),
        margin: UiRect::all(px(UI_SCALE*15.)),
        ..default()
    }
}

pub fn button(text: impl Into<String>, asset_server: &AssetServer) -> impl Bundle {
    (
        Button,
        children![(
            Node {
                left: px(UI_SCALE*15.),
                top: px(UI_SCALE*15.),
                bottom: px(UI_SCALE*15.),
                ..default()
            },
            Text::new(text),
            TextFont {
                font: if let Some(font_path) = text::BUTTON_FONT_PATH { asset_server.load(font_path) } else { default() },
                font_size: text::BUTTON_FONT_SIZE*UI_SCALE,
                ..default()
            }
        )],
        BackgroundColor(button::STANDARD_COLOR),
    )
}

pub fn title_text(text: impl Into<String>, asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: if let Some(font_path) = text::TITLE_FONT_PATH { asset_server.load(font_path) } else { default() },
            font_size: text::TITLE_FONT_SIZE*UI_SCALE,
            ..default()
        }
    )
}
