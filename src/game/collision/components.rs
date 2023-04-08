use bevy::{
    prelude::{Component, Vec2},
    sprite::collide_aabb::Collision,
};

#[derive(Component)]
pub struct Collidable {
    pub is_solid: bool,
    pub size: Vec2,
    pub collision: CollisionData,
}

pub struct CollisionData {
    pub is_collided: bool,
    pub collision_side: Vec<Collision>,
}
