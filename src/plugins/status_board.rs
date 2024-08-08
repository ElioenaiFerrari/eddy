use bevy::prelude::*;

use crate::prelude::{Health, HealthPotions, Player, StatusBoard};

fn health_color(health: u16) -> Color {
    match health {
        0..=25 => Color::srgb(0.8, 0.0, 0.0),
        26..=50 => Color::srgb(0.8, 0.4, 0.0),
        51..=75 => Color::srgb(0.8, 0.8, 0.0),
        _ => Color::srgb(0.0, 0.8, 0.0),
    }
}

fn health_potions_color(health_potions: u8) -> Color {
    match health_potions {
        0 => Color::srgb(0.8, 0.0, 0.0),
        1..=3 => Color::srgb(0.8, 0.8, 0.0),
        _ => Color::srgb(0.0, 0.8, 0.0),
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        StatusBoard,
        TextBundle::from_sections([
            TextSection::new(
                "Health: ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));

    commands.spawn((
        StatusBoard,
        TextBundle::from_sections([
            TextSection::new(
                "Health Potions: ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));
}

fn update_status_board(
    mut query_status_board: Query<&mut Text, With<StatusBoard>>,
    query_player: Query<(&Health, &HealthPotions), With<Player>>,
) {
    for (i, mut status_board) in query_status_board.iter_mut().enumerate() {
        let (health, health_potions) = query_player.single();

        if i == 0 {
            log::info!("Health: {:?}", health.0);
            status_board.sections[1].value = health.0.to_string();
            status_board.sections[1].style.color = health_color(health.0);
        } else {
            log::info!("Health Potions: {:?}", health_potions.0);
            status_board.sections[1].value = health_potions.0.to_string();
            status_board.sections[1].style.color = health_potions_color(health_potions.0);
        }
    }
}

pub struct StatusBoardPlugin;

impl Plugin for StatusBoardPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Building StatusBoardPlugin");
        app.add_systems(PostStartup, setup)
            .add_systems(Update, update_status_board);
    }
}
