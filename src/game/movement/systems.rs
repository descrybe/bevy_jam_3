use bevy::{
    prelude::{Query, Transform, Vec3, With},
    sprite::collide_aabb::Collision,
};

use crate::game::collision::components::{Collidable, Solid};

fn get_reaction(collision: &Collision) -> Vec3 {
    match collision {
        Collision::Left => Vec3::X,
        Collision::Right => Vec3::NEG_X,
        Collision::Top => Vec3::NEG_Y,
        Collision::Bottom => Vec3::Y,
        Collision::Inside => Vec3::ZERO,
    }
}

pub fn adjust_movement(mut query: Query<(&mut Transform, &Collidable, &Solid), With<Solid>>) {
    for (mut transform, collider, solidity) in query.iter_mut() {
        if !collider.collision.is_collided {
            transform.translation = solidity.target_point;
            continue;
        }
        let reaction_vector = collider
            .collision
            .collision_side
            .iter()
            .map(|side| get_reaction(side))
            .sum::<Vec3>()
            .normalize_or_zero();
        let origin_transform = solidity.target_point - transform.translation;
        let length = origin_transform.length();
        let normalized_result =
            solidity.target_point.normalize_or_zero() + reaction_vector.normalize_or_zero();

        transform.translation += (solidity.collision_impact * normalized_result * length)
            + (1.0 - solidity.collision_impact) * origin_transform;
    }
}
