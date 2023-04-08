use bevy::prelude::Entity;

pub struct DamageEvent {
    pub damage_amount: u32,
    pub target: Entity,
}
