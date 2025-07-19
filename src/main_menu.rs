use bevy::prelude::*;

mod plugin {
    use crate::GameState;
    use bevy::prelude::*;

    struct MainMenuPlugin;
    impl Plugin for MainMenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::MainMenu), setup);
            //app.add_systems()
        }
    }
    fn setup(mut commands: Commands) {
        info!("Starting main menu plugin");
    }
}
