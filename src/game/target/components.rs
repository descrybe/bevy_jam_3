use crate::Entity;
use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct TargetHolderComponent {
    pub target_entity: Entity,
}

#[derive(Component)]
pub struct DirectionHolderComponent {
    pub direction: Vec2,
}
