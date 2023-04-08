use bevy::prelude::{Vec2, Component};

#[derive(Component)]
pub struct Flight {
    pub speed: f32,
    pub direction: Vec2
}
