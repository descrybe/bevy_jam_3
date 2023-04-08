use bevy::prelude::Entity;

pub struct DamageEvent {
    pub damage_amount: i32,
    pub target: Entity,
}
