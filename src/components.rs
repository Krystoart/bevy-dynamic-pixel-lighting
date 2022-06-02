use crate::TIME_STEP;
use bevy::prelude::{Component, Timer};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct PlayerTimer(pub Timer);

impl Default for PlayerTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, true))
    }
}
