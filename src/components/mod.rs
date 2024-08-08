use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Gravity(pub f32);

#[derive(Debug, Component)]
pub struct Acceleration(pub Vec2);

#[derive(Default, Component)]
pub struct Player;

#[derive(Component)]
pub struct EddyAnimationIndices {
    pub walk: Vec<usize>,
    pub run: Vec<usize>,
    pub drunk: Vec<usize>,
    pub idle: Vec<usize>,
    pub jump: Vec<usize>,
    pub attack: Vec<usize>,
    pub special: Vec<usize>,
    pub angry: Vec<usize>,
}

#[derive(Default, Debug, Component)]
pub struct Xp(pub u32);

#[derive(Default, Debug, Component)]
pub struct Level(pub u32);

#[derive(Default, Debug, Component)]
pub struct Username(pub String);

#[derive(Debug, Component)]
pub struct HealthPotions(pub u8);

#[derive(Default, Debug, Component)]
pub struct Health(pub u16);

#[derive(Default, Debug, Component)]
pub struct Jumping(pub bool);

#[derive(Debug, Component)]
pub struct StatusBoard;
