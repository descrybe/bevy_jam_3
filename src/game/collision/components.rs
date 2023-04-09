use bevy::{
    prelude::{Component, Vec2, Vec3},
    sprite::collide_aabb::Collision,
};

#[derive(Component)]
pub struct Solid {
    pub target_point: Vec3,
    pub collision_impact: f32,
}

#[derive(Component)]
pub struct Collidable {
    pub size: Vec2,
    pub collision: CollisionData,
}

pub struct CollisionData {
    pub is_collided: bool,
    pub collision_side: Vec<Collision>,
}
