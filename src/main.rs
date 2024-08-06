use bevy::prelude::*;
use eddy::prelude::*;

fn game_acceleration(
    mut acceleration_timer: ResMut<AccelerationTimer>,
    time: Res<Time>,
    mut query_game: Query<&mut Acceleration>,
) {
    if acceleration_timer.0.tick(time.delta()).just_finished() {
        let mut acceleration = query_game.single_mut();

        acceleration.0 = Vec2::new(acceleration.0.x + 1., acceleration.0.y + 1.);

        println!("Game acceleration: {:#?}", acceleration.0);
    }
}

fn is_not_ended(query_player: Query<&Health, With<Player>>) -> bool {
    let player_health = query_player.single();

    player_health.0 > 0
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins(GamePlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(
            Update,
            (
                game_acceleration.run_if(is_not_ended),
                // player_jump.run_if(is_player_in_air),
            )
                .run_if(is_not_ended),
        )
        .run();
}
