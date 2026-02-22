const SPLASH_SCREEN_DURATION: f32 = 3.;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::screens::Screen;

pub struct SplashScreenPlugin;

struct VanishTimer(Timer);

impl Default for VanishTimer {
    fn default() -> Self {
        VanishTimer(Timer::from_seconds(SPLASH_SCREEN_DURATION, TimerMode::Once))
    }
}

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::SplashScreen), spawn_splash_screen)
            .add_systems(
                Update,
                (splash_screen_timer.run_if(in_state(Screen::SplashScreen)),),
            );
    }
}

//TODO: maybe only one camera??
#[derive(Component)]
#[require(Camera2d)]
pub struct SplashScreenCamera;

fn spawn_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((DespawnOnExit(Screen::SplashScreen), SplashScreenCamera));
    commands.spawn((
        DespawnOnExit(Screen::SplashScreen),
        Node {
            margin: UiRect::all(Val::Auto),
            //this needs to be tuned later
            width: Val::Percent(70.),

            ..Default::default()
        },
        ImageNode::new(asset_server.load("bevy_logo_dark.png")),
    ));

    info!("spawned splash screen");
}

fn splash_screen_timer(
    time: Res<Time>,
    mut timer: Local<VanishTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        next_screen.set(Screen::TitleScreen);
        info!("timer is finished, switching to title screen");
    }
}
