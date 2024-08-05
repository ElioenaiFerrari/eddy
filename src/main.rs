use bevy::prelude::*;

// Character Components
#[derive(Default, Debug, Component)]
struct Xp(u32);

#[derive(Default, Debug, Component)]
struct Level(u32);

#[derive(Default, Debug, Component)]
struct Username(String);

#[derive(Default, Debug, Component)]
struct Health(u16);

#[derive(Default, Debug, Component)]
struct Jumping(bool);

#[derive(Debug, Bundle)]
struct CharacterBundle {
    xp: Xp,
    level: Level,
    name: Username,
    health: Health,
    jumping: Jumping,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            xp: Xp(0),
            level: Level(1),
            name: Username("Player".to_string()),
            health: Health(100),
            jumping: Jumping(false),
        }
    }
}

#[derive(Debug, Resource)]
struct GravityTimer(Timer);

#[derive(Debug, Resource)]
struct JumpTimer(Timer);

#[derive(Debug, Resource)]
struct AnimationTimer(Timer);

#[derive(Debug, Resource)]
struct AccelerationTimer(Timer);

#[derive(Debug, Component)]
struct Gravity(f32);

#[derive(Debug, Component)]
struct Acceleration(Vec2);

#[derive(Debug, Bundle)]
struct GameBundle {
    gravity: Gravity,
    acceleration: Acceleration,
}

impl Default for GameBundle {
    fn default() -> Self {
        GameBundle {
            gravity: Gravity(9.8),
            acceleration: Acceleration(Vec2::new(0.5, 0.5)),
        }
    }
}

#[derive(Default, Component)]
struct Player;

#[derive(Component)]
struct EddyAnimationIndices {
    walk: Vec<usize>,
    run: Vec<usize>,
    drunk: Vec<usize>,
    idle: Vec<usize>,
    jump: Vec<usize>,
    attack: Vec<usize>,
}

#[derive(Default, Component)]
struct Enemy;

fn setup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(GameBundle::default());
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = assets_server.load("sprite/eddy.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(128), 16, 10, None, None);
    let texture_atlas_handler = layouts.add(texture_atlas);

    let animation_indices = EddyAnimationIndices {
        idle: vec![0, 5],
        drunk: vec![7, 11],
        walk: vec![33, 39],
        run: vec![17, 23],
        jump: vec![50, 80],
        attack: vec![30, 36],
    };

    commands.spawn((
        CharacterBundle::default(),
        Player,
        SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
        TextureAtlas {
            index: animation_indices.idle[0],
            layout: texture_atlas_handler,
        },
        animation_indices,
    ));
    commands.spawn((
        CharacterBundle {
            name: Username("Goblin".to_string()),
            ..default()
        },
        Enemy,
        SpriteBundle::default(),
    ));

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

fn is_not_ended(
    query_player: Query<&Health, With<Player>>,
    query_enemy: Query<&Health, With<Enemy>>,
) -> bool {
    let player_health = query_player.single();
    let enemy_health = query_enemy.single();

    player_health.0 > 0 && enemy_health.0 > 0
}

fn is_player_not_movimenting(
    keys: Res<ButtonInput<KeyCode>>,
    query_player: Query<(&Transform, &Jumping), With<Player>>,
) -> bool {
    let (transform, jumping) = query_player.single();

    let is_pressed = keys.is_changed();

    !is_pressed && !jumping.0 && !transform.translation.is_normalized()
}

fn idle_animation(
    mut query_player: Query<(&mut TextureAtlas, &EddyAnimationIndices), With<Player>>,
    mut animation_timer: ResMut<AnimationTimer>,
    time: Res<Time>,
) {
    let (mut atlas, animation_indices) = query_player.single_mut();
    let first_idle = animation_indices.idle[0];
    let last_idle = animation_indices.idle[1];

    if atlas.index < first_idle || atlas.index > last_idle {
        atlas.index = first_idle;
    }

    if animation_timer.0.tick(time.delta()).just_finished() {
        atlas.index = if atlas.index == last_idle {
            first_idle
        } else {
            atlas.index + 1
        };
    }
}

fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    query_game: Query<&Acceleration>,
    mut animation_timer: ResMut<AnimationTimer>,
    time: Res<Time>,
    mut query_player: Query<
        (
            &mut Health,
            &mut Jumping,
            &mut Transform,
            &mut EddyAnimationIndices,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
) {
    let acceleration = query_game.single();
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::KeyE => {
                for (mut health, _, _, animation_indices, mut atlas) in query_player.iter_mut() {
                    println!("Player drank!");
                    health.0 += 10;

                    let first_drunk = animation_indices.drunk[0];
                    let last_drunk = animation_indices.drunk[1];

                    if animation_timer.0.tick(time.delta()).just_finished() {
                        atlas.index = if atlas.index == last_drunk {
                            first_drunk
                        } else {
                            atlas.index + 1
                        };
                    }

                    println!("Player attacked!");
                }
            }

            KeyCode::KeyW => {
                for (_, mut jumping, _, _, _) in query_player.iter_mut() {
                    if !jumping.0 {
                        jumping.0 = true;
                    }
                }
            }
            _ => {}
        }
    }

    for key in keys.get_pressed() {
        match key {
            KeyCode::KeyA => {
                for (_, jumping, mut transform, animation_indices, mut atlas) in
                    query_player.iter_mut()
                {
                    transform.rotation = Quat::from_rotation_y(3.14);

                    if !jumping.0 {
                        let first_walk = animation_indices.walk[0];
                        let last_walk = animation_indices.walk[1];
                        if atlas.index < first_walk || atlas.index > last_walk {
                            atlas.index = first_walk;
                        }

                        if animation_timer.0.tick(time.delta()).just_finished() {
                            atlas.index = if atlas.index == last_walk {
                                first_walk
                            } else {
                                atlas.index + 1
                            };
                        }
                    } else {
                        let first_jump = animation_indices.jump[0];
                        let last_jump = animation_indices.jump[1];

                        if atlas.index < first_jump || atlas.index > last_jump {
                            atlas.index = first_jump;
                        }

                        if animation_timer.0.tick(time.delta()).just_finished() {
                            atlas.index = if atlas.index == last_jump {
                                first_jump
                            } else {
                                atlas.index + 1
                            };
                        }
                    }
                    transform.translation.x -= 10. * acceleration.0.x;

                    // Virar personagem para esquerda
                }
            }

            KeyCode::KeyD => {
                for (_, jumping, mut transform, animation_indices, mut atlas) in
                    query_player.iter_mut()
                {
                    transform.rotation = Quat::from_rotation_y(0.);
                    if !jumping.0 {
                        let first_walk = animation_indices.walk[0];
                        let last_walk = animation_indices.walk[1];
                        if atlas.index < first_walk || atlas.index > last_walk {
                            atlas.index = first_walk;
                        }
                        if animation_timer.0.tick(time.delta()).just_finished() {
                            atlas.index = if atlas.index == last_walk {
                                first_walk
                            } else {
                                atlas.index + 1
                            };
                        }
                    } else {
                        let first_jump = animation_indices.jump[0];
                        let last_jump = animation_indices.jump[1];
                        if atlas.index < first_jump || atlas.index > last_jump {
                            atlas.index = first_jump;
                        }
                        if animation_timer.0.tick(time.delta()).just_finished() {
                            atlas.index = if atlas.index == last_jump {
                                first_jump
                            } else {
                                atlas.index + 1
                            };
                        }
                    }
                    transform.translation.x += 10. * acceleration.0.x;
                }
            }

            _ => {}
        }
    }
}

fn player_jump(
    mut jump_timer: ResMut<JumpTimer>,
    mut animation_timer: ResMut<AnimationTimer>,
    mut query_player: Query<
        (&mut Transform, &EddyAnimationIndices, &mut TextureAtlas),
        With<Player>,
    >,
    time: Res<Time>,
    query_game: Query<&Gravity>,
) {
    let (mut transform, animation_indices, mut atlas) = query_player.single_mut();
    let gravity = query_game.single();

    if jump_timer.0.tick(time.delta()).just_finished() {
        let first_jump = animation_indices.jump[0];
        let last_jump = animation_indices.jump[1];

        transform.translation.y += gravity.0 * 30.;
        if animation_timer.0.tick(time.delta()).just_finished() {
            atlas.index = if atlas.index == last_jump {
                first_jump
            } else {
                atlas.index + 1
            };
        }
    }
}

fn player_jump_back(
    mut query_player: Query<
        (
            &mut Jumping,
            &mut Transform,
            &EddyAnimationIndices,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
    query_game: Query<&Gravity>,
    time: Res<Time>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut animation_timer: ResMut<AnimationTimer>,
) {
    let (mut jumping, mut transform, aimation_indices, mut atlas) = query_player.single_mut();
    let gravity = query_game.single();
    if gravity_timer.0.tick(time.delta()).just_finished() {
        transform.translation.y -= gravity.0;

        let first_jump = aimation_indices.jump[0];
        let last_jump = aimation_indices.jump[1];
        if animation_timer.0.tick(time.delta()).just_finished() {
            atlas.index = if atlas.index == last_jump {
                first_jump
            } else {
                atlas.index + 1
            };
        }

        if transform.translation.y <= 0. {
            jumping.0 = false;
            transform.translation.y = 0.;
            atlas.index = aimation_indices.walk[0];
        }
    }
}

fn is_player_jumping(query_player: Query<&Jumping, With<Player>>) -> bool {
    let jumping = query_player.single();

    jumping.0
}

fn is_player_in_floor(query_player: Query<&Transform, With<Player>>) -> bool {
    let transform = query_player.single();

    transform.translation.y == 0.0
}

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                game_acceleration.run_if(is_not_ended),
                idle_animation.run_if(is_player_not_movimenting),
                player_move,
                player_jump_back.run_if(is_player_jumping),
                player_jump
                    .run_if(is_player_in_floor)
                    .run_if(is_player_jumping),
                // player_jump.run_if(is_player_in_air),
            )
                .run_if(is_not_ended),
        )
        .run();
}
