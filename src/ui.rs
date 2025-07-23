pub mod colors {
    use bevy::color::Color;
    use bevy::prelude::*;

    pub const RED: Color = Color::Srgba(Srgba::new(1.0, 0.0, 0.0, 0.8));
}

pub mod buttons {
    use super::colors::*;
    use bevy::prelude::*;

    //some parts of the button code copied from an official bevy example
    pub fn main_button<S: Into<String>>(
        asset_server: &AssetServer,
        text: S,
    ) -> impl Bundle + use<S> {
        (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(130.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(RED),
                children![(
                    Text::new(text),
                    TextFont {
                        font: asset_server.load("fonts/arial_narrow_7.ttf"),
                        font_size: 66.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            )],
        )
    }
}

pub mod text {
    use bevy::color::palettes::basic::BLACK;
    use bevy::prelude::*;
    pub fn main_title<S: Into<String>>(
        asset_server: &AssetServer,
        text: S,
    ) -> impl Bundle + use<S> {
        (
            Node {
                border: UiRect::top(Val::Px(50.)),
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                //align_items: AlignItems::Center,
                //align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            children![(
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/arial_narrow_7.ttf"),
                    font_size: 100.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )],
        )
    }

    pub fn loading_screen<S: Into<String>>(
        asset_server: &AssetServer,
        text: S,
    ) -> impl Bundle + use<S> {
        (
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
        children![(
                Text::new(text),
                TextFont {
                    font: asset_server.load("fonts/arial_narrow_7.ttf"),
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )],
            )
    }
}

pub mod logo {
    use bevy::prelude::*;
    pub fn bevy_logo(asset_server: &AssetServer) -> impl Bundle + use<> {
        (
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            Sprite {
            image: asset_server.load("bevy/logo_dark.png"),
            image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
            ..Default::default()
        })
    }
}
