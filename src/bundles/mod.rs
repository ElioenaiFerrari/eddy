use bevy::prelude::*;

use crate::prelude::{Acceleration, Gravity, Health, HealthPotions, Jumping, Level, Username, Xp};

#[derive(Debug, Bundle)]
pub struct CharacterBundle {
    pub xp: Xp,
    pub level: Level,
    pub name: Username,
    pub health: Health,
    pub jumping: Jumping,
    pub health_potions: HealthPotions,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            xp: Xp(0),
            level: Level(1),
            name: Username("Player".to_string()),
            health: Health(100),
            jumping: Jumping(false),
            health_potions: HealthPotions(2),
        }
    }
}

#[derive(Debug, Bundle)]
pub struct GameBundle {
    pub gravity: Gravity,
    pub acceleration: Acceleration,
}

impl Default for GameBundle {
    fn default() -> Self {
        GameBundle {
            gravity: Gravity(9.8),
            acceleration: Acceleration(Vec2::new(0.5, 0.5)),
        }
    }
}
