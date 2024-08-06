use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct GravityTimer(pub Timer);

#[derive(Debug, Resource)]
pub struct JumpTimer(pub Timer);

#[derive(Debug, Resource)]
pub struct AnimationTimer(pub Timer);

#[derive(Debug, Resource)]
pub struct AccelerationTimer(pub Timer);
