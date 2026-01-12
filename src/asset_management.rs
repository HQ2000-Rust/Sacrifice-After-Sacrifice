use bevy::prelude::*;

pub mod loading_screen {
    use bevy::prelude::*;

    #[derive(Resource, TypePath)]
    pub struct BevyLogo(pub Handle<Image>);
}
