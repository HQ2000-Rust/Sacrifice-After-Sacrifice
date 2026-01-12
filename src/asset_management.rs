use bevy::prelude::*;

pub mod loading_screen {
    use bevy::prelude::*;

    #[derive(Resource)]
    pub struct BevyLogo(pub Handle<Image>);
}
