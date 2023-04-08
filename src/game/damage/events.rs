use bevy::prelude::Entity;

pub struct DamageEvent {
    pub dealer: Entity,
    pub target: Entity,
}