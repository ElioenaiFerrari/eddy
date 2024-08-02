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
            acceleration: Acceleration(Vec2::new(1., 1.)),
        }
    }
}

#[derive(Default, Component)]
struct Player;

#[derive(Component)]
struct AnimationIndices {
    walk: usize,
    run: usize,
    idle: usize,
    jump: usize,
    attack: usize,
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

    let texture_handle: Handle<Image> = assets_server.load("sprite/idle_3.png");
    let texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::new(80, 100), 7, 1, None, Some(UVec2::new(84, 30)));
    let texture_atlas_handler = layouts.add(texture_atlas);

    let animation_indices = AnimationIndices {
        idle: 1,
        walk: 1,
        run: 2,
        jump: 3,
        attack: 4,
    };

    commands.spawn((
        CharacterBundle::default(),
        Player,
        SpriteBundle {
            texture: texture_handle,
            sprite: Sprite {
                custom_size: Some(Vec2::new(160., 200.)),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            index: animation_indices.idle,
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
        0.01,
        TimerMode::Repeating,
    )));
    commands.insert_resource(JumpTimer(Timer::from_seconds(0.001, TimerMode::Repeating)));
    commands.insert_resource(AccelerationTimer(Timer::from_seconds(
        30.0,
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

fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    query_game: Query<&Acceleration>,
    mut param_set: ParamSet<(
        Query<&mut Health, With<Enemy>>,
        Query<
            (
                &mut Xp,
                &mut Jumping,
                &mut Transform,
                &mut AnimationIndices,
                &mut TextureAtlas,
            ),
            With<Player>,
        >,
    )>,
) {
    let acceleration = query_game.single();
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::KeyE => {
                for mut health in param_set.p0().iter_mut() {
                    health.0 -= 10;

                    println!("Player attacked!");
                }
            }

            KeyCode::KeyW => {
                for (_, mut jumping, _, _, _) in param_set.p1().iter_mut() {
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
                for (_, _, mut transform, animation_indices, mut atlas) in param_set.p1().iter_mut()
                {
                    atlas.index = animation_indices.walk;
                    transform.translation.x -= 10. * acceleration.0.x;
                    transform.rotation = Quat::from_rotation_y(3.14);
                    // Virar personagem para esquerda
                    println!("Player moved left! {}", transform.translation.x);
                }
            }

            KeyCode::KeyD => {
                for (_, _, mut transform, animation_indices, mut atlas) in param_set.p1().iter_mut()
                {
                    atlas.index = animation_indices.walk;
                    transform.translation.x += 10. * acceleration.0.x;
                    transform.rotation = Quat::from_rotation_y(0.);
                    println!("Player moved right! {}", transform.translation.x);
                }
            }

            _ => {}
        }
    }
}

fn player_jump(
    mut jump_timer: ResMut<JumpTimer>,
    mut query_player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    query_game: Query<&Gravity>,
) {
    if jump_timer.0.tick(time.delta()).just_finished() {
        let mut transform = query_player.single_mut();
        let gravity = query_game.single();

        transform.translation.y += gravity.0 * 30.;
    }
}

fn player_jump_back(
    mut query_player: Query<(&mut Jumping, &mut Transform), With<Player>>,
    query_game: Query<&Gravity>,
    time: Res<Time>,
    mut gravity_timer: ResMut<GravityTimer>,
) {
    let (mut jumping, mut transform) = query_player.single_mut();
    let gravity = query_game.single();
    if gravity_timer.0.tick(time.delta()).just_finished() {
        transform.translation.y -= gravity.0;

        if transform.translation.y <= 0. {
            jumping.0 = false;
            transform.translation.y = 0.;
        }
    }
}

fn enemy_loop(query_enemy: Query<(&Xp, &Health), With<Enemy>>) {
    let (xp, health) = query_enemy.single();

    // println!("Enemy XP: {:#?}", xp);
    // println!("Enemy Health: {:#?}", health);

    // println!("Enemy attacked!");
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
                player_move,
                player_jump_back.run_if(is_player_jumping),
                player_jump
                    .run_if(is_player_in_floor)
                    .run_if(is_player_jumping),
                // player_jump.run_if(is_player_in_air),
                enemy_loop,
            )
                .run_if(is_not_ended),
        )
        .run();
}
