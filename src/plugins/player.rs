use crate::prelude::*;
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = assets_server.load("sprite/eddy.png");

    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(128), 16, 10, None, None);
    let texture_atlas_handler = layouts.add(texture_atlas);

    let animation_indices = EddyAnimationIndices {
        idle: vec![0, 5],
        drunk: vec![16, 26],
        walk: vec![33, 39],
        run: vec![50, 55],
        jump: vec![55, 60],
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
}

fn is_player_not_movimenting(
    keys: Res<ButtonInput<KeyCode>>,
    query_player: Query<&Jumping, With<Player>>,
) -> bool {
    let jumping = query_player.single();

    let just_pressed = keys.get_just_pressed();
    let pressed = keys.get_pressed();

    return just_pressed.len() == 0 && pressed.len() == 0 && !jumping.0;
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
    mut query_game: Query<&mut Acceleration>,
    mut animation_timer: ResMut<AnimationTimer>,
    time: Res<Time>,
    mut query_player: Query<
        (
            &mut Health,
            &mut Jumping,
            &mut Transform,
            &mut EddyAnimationIndices,
            &mut TextureAtlas,
            &mut HealthPotions,
        ),
        With<Player>,
    >,
) {
    let mut acceleration = query_game.single_mut();
    let is_running = keys.pressed(KeyCode::ShiftLeft);
    if is_running {
        acceleration.0 = Vec2::new(1.5, 1.5);
    } else {
        acceleration.0 = Vec2::new(0.5, 0.5);
    }

    for key in keys.get_just_pressed() {
        match key {
            KeyCode::KeyW => {
                for (_, mut jumping, _, _, _, _) in query_player.iter_mut() {
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
            KeyCode::KeyE => {
                for (mut health, _, _, animation_indices, mut atlas, mut health_potions) in
                    query_player.iter_mut()
                {
                    if health_potions.0 > 0 {
                        let first_drunk = animation_indices.drunk[0];
                        let last_drunk = animation_indices.drunk[1];

                        if atlas.index < first_drunk || atlas.index > last_drunk {
                            atlas.index = first_drunk;
                        }

                        if animation_timer.0.tick(time.delta()).just_finished() {
                            atlas.index = if atlas.index == last_drunk {
                                health.0 += 10;
                                health_potions.0 -= 1;
                                first_drunk
                            } else {
                                atlas.index + 1
                            };
                        }
                    }
                }
            }

            KeyCode::KeyA => {
                for (_, jumping, mut transform, animation_indices, mut atlas, _) in
                    query_player.iter_mut()
                {
                    transform.rotation = Quat::from_rotation_y(3.14);

                    if !jumping.0 {
                        if is_running {
                            let first_run = animation_indices.run[0];
                            let last_run = animation_indices.run[1];
                            if atlas.index < first_run || atlas.index > last_run {
                                atlas.index = first_run;
                            }

                            if animation_timer.0.tick(time.delta()).just_finished() {
                                atlas.index = if atlas.index == last_run {
                                    first_run
                                } else {
                                    atlas.index + 1
                                };
                            }
                        } else {
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
                for (_, jumping, mut transform, animation_indices, mut atlas, _) in
                    query_player.iter_mut()
                {
                    transform.rotation = Quat::from_rotation_y(0.);
                    if !jumping.0 {
                        if is_running {
                            let first_run = animation_indices.run[0];
                            let last_run = animation_indices.run[1];
                            if atlas.index < first_run || atlas.index > last_run {
                                atlas.index = first_run;
                            }

                            if animation_timer.0.tick(time.delta()).just_finished() {
                                atlas.index = if atlas.index == last_run {
                                    first_run
                                } else {
                                    atlas.index + 1
                                };
                            }
                        } else {
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

fn is_not_ended(query_player: Query<&Health, With<Player>>) -> bool {
    let player_health = query_player.single();

    player_health.0 > 0
}

fn is_player_jumping(query_player: Query<&Jumping, With<Player>>) -> bool {
    let jumping = query_player.single();

    jumping.0
}

fn is_player_in_floor(query_player: Query<&Transform, With<Player>>) -> bool {
    let transform = query_player.single();

    transform.translation.y == 0.0
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                idle_animation.run_if(is_player_not_movimenting),
                player_move.run_if(is_not_ended),
                player_jump_back.run_if(is_player_jumping),
                player_jump
                    .run_if(is_player_in_floor)
                    .run_if(is_player_jumping),
            ),
        );
    }
}
