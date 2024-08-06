use crate::prelude::*;
use bevy::prelude::*;

pub struct GamePlugin;

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn(GameBundle::default());
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: assets_server.load("sprite/background.png"),
        ..default()
    });

    commands.insert_resource(GravityTimer(Timer::from_seconds(
        0.05,
        TimerMode::Repeating,
    )));
    commands.insert_resource(JumpTimer(Timer::from_seconds(0.001, TimerMode::Repeating)));
    commands.insert_resource(AccelerationTimer(Timer::from_seconds(
        30.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(AnimationTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
