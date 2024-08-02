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

#[derive(Debug, Bundle)]
struct CharacterBundle {
    xp: Xp,
    level: Level,
    name: Username,
    health: Health,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            xp: Xp(0),
            level: Level(1),
            name: Username("Player".to_string()),
            health: Health(100),
        }
    }
}

// Game Components
#[derive(Debug, Component)]
enum Turn {
    Player,
    Enemy,
}

#[derive(Debug, Resource)]
struct Clock(Timer);

#[derive(Debug, Component)]
struct Gravity(f32);

#[derive(Debug, Bundle)]
struct GameBundle {
    turn: Turn,
    gravity: Gravity,
}

impl Default for GameBundle {
    fn default() -> Self {
        GameBundle {
            turn: Turn::Player,
            gravity: Gravity(9.8),
        }
    }
}

#[derive(Default, Component)]
struct Player;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Default, Component)]
struct Enemy;

fn setup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(GameBundle::default());
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = assets_server.load("sprite/idle_3.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(40, 120), 11, 1, None, None);
    let texture_atlas_handler = layouts.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 1, last: 6 };

    commands.spawn((
        CharacterBundle::default(),
        Player,
        SpriteBundle {
            texture: texture_handle,
            sprite: Sprite {
                custom_size: Some(Vec2::new(200., 256.)),
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            index: animation_indices.first,
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

    commands.insert_resource(Clock(Timer::from_seconds(1.0, TimerMode::Repeating)));
}

fn is_player_turn(query_game: Query<&Turn>) -> bool {
    let turn = query_game.single();

    match turn {
        Turn::Player => true,
        Turn::Enemy => false,
    }
}

fn is_enemy_turn(query_game: Query<&Turn>) -> bool {
    let turn = query_game.single();

    match turn {
        Turn::Player => false,
        Turn::Enemy => true,
    }
}

fn is_not_ended(
    query_player: Query<&Health, With<Player>>,
    query_enemy: Query<&Health, With<Enemy>>,
) -> bool {
    let player_health = query_player.single();
    let enemy_health = query_enemy.single();

    player_health.0 > 0 && enemy_health.0 > 0
}

fn player_loop(
    keys: Res<ButtonInput<KeyCode>>,
    mut query_game: Query<(&mut Turn, &Gravity)>,
    mut param_set: ParamSet<(
        Query<&mut Health, With<Enemy>>,
        Query<
            (
                &mut Xp,
                &mut Transform,
                &mut AnimationIndices,
                &mut TextureAtlas,
            ),
            With<Player>,
        >,
    )>,
    time: Res<Time>,
) {
    let (mut turn, gravity) = query_game.single_mut();

    for key in keys.get_just_pressed() {
        match key {
            KeyCode::KeyE => {
                for mut health in param_set.p0().iter_mut() {
                    health.0 -= 10;

                    println!("Player attacked!");
                }

                *turn = Turn::Enemy;
            }

            KeyCode::KeyW => {
                for (_, mut transform, _, _) in param_set.p1().iter_mut() {
                    transform.translation.y += gravity.0 * 10.0;
                    println!("Player moved up! {}", transform.translation.y);
                }
            }
            _ => {}
        }
    }

    for key in keys.get_pressed() {
        match key {
            KeyCode::KeyA => {
                for (_, mut transform, animation_indices, mut atlas) in param_set.p1().iter_mut() {
                    atlas.index = animation_indices.last;
                    transform.translation.x -= 8.0;
                    // Virar personagem para esquerda
                    println!("Player moved left! {}", transform.translation.x);
                }
            }

            KeyCode::KeyD => {
                for (_, mut transform, _, _) in param_set.p1().iter_mut() {
                    transform.translation.x += 8.0;
                    println!("Player moved right! {}", transform.translation.x);
                }
            }

            _ => {}
        }
    }
}

fn player_jump(
    mut clock: ResMut<Clock>,
    mut query_player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    query_game: Query<&Gravity>,
) {
    clock.0.tick(time.delta());
    let mut transform = query_player.single_mut();
    let gravity = query_game.single();

    transform.translation.y += gravity.0 * time.delta_seconds() * 30.0;
}

fn player_jump_back(
    mut query_player: Query<&mut Transform, With<Player>>,
    query_game: Query<&Gravity>,
    time: Res<Time>,
    mut clock: ResMut<Clock>,
) {
    clock.0.tick(time.delta());
    let mut transform = query_player.single_mut();
    let gravity = query_game.single();

    transform.translation.y -= gravity.0;
}

fn enemy_loop(query_enemy: Query<(&Xp, &Health), With<Enemy>>, mut query_game: Query<&mut Turn>) {
    let (xp, health) = query_enemy.single();

    println!("Enemy XP: {:#?}", xp);
    println!("Enemy Health: {:#?}", health);

    let mut turn = query_game.single_mut();
    println!("Enemy attacked!");
    *turn = Turn::Player;
}

fn is_player_in_air(query_player: Query<&Transform, With<Player>>) -> bool {
    let transform = query_player.single();

    transform.translation.y > 0.0
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_loop.run_if(is_player_turn),
                player_jump_back.run_if(is_player_in_air),
                // player_jump.run_if(is_player_in_air),
                enemy_loop.run_if(is_enemy_turn),
            )
                .run_if(is_not_ended),
        )
        .run();
}
