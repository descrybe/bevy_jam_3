use bevy::{prelude::Component, prelude::Vec2};

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}