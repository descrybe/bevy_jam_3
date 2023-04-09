use bevy::prelude::Entity;

pub struct LaunchBulletEvent {
    pub owner: Entity,
    pub target: Entity,
}
