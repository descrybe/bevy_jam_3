use bevy::prelude::Entity;

pub struct CollisionEvent {
    first: Entity,
    second: Entity,
}

impl CollisionEvent {
    pub fn new(first: Entity, second: Entity) -> Self {
        CollisionEvent { first, second }
    }

    pub fn first(&self) -> &Entity {
        &self.first
    }

    pub fn second(&self) -> &Entity {
        &self.second
    }
}
