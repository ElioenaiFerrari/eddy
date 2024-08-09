use crate::prelude::*;
use bevy::prelude::*;

pub struct GamePlugin;

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    log::info!("Setting up GameBundle");
    commands.spawn(GameBundle::default());

    log::info!("Setting up CharacterBundle");
    commands.spawn(Camera2dBundle::default());

    log::info!("Setting up background");
    commands.spawn(SpriteBundle {
        texture: assets_server.load("sprite/background.png"),
        ..default()
    });

    log::info!("Setting up GravityTimer");
    commands.insert_resource(GravityTimer(Timer::from_seconds(
        0.005,
        TimerMode::Repeating,
    )));

    log::info!("Setting up JumpTimer");
    commands.insert_resource(JumpTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

    log::info!("Setting up AccelerationTimer");
    commands.insert_resource(AccelerationTimer(Timer::from_seconds(
        30.0,
        TimerMode::Repeating,
    )));

    log::info!("Setting up AnimationTimer");
    commands.insert_resource(AnimationTimer(Timer::from_seconds(
        0.05,
        TimerMode::Repeating,
    )));
}

fn game_acceleration(
    mut acceleration_timer: ResMut<AccelerationTimer>,
    time: Res<Time>,
    mut query_game: Query<&mut Acceleration>,
) {
    if acceleration_timer.0.tick(time.delta()).just_finished() {
        let mut acceleration = query_game.single_mut();

        acceleration.0 = Vec2::new(acceleration.0.x + 1., acceleration.0.y + 1.);
        log::info!("Acceleration: {:?}", acceleration.0);
    }
}

fn is_not_ended(query_player: Query<&Health, With<Player>>) -> bool {
    let player_health = query_player.single();

    player_health.0 > 0
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        log::info!("Building GamePlugin");
        app.add_systems(Startup, setup).add_systems(
            Update,
            game_acceleration.run_if(is_not_ended),
            // player_jump.run_if(is_player_in_air),
        );
    }
}
