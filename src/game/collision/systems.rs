use bevy::{
    prelude::{Component, Entity, EventReader, EventWriter, Query, Transform, With, Without},
    sprite::collide_aabb::{collide, Collision},
};

use super::{components::Collidable, events::CollisionEvent};

pub fn clear_collision(mut collision_query: Query<&mut Collidable>) {
    for mut collider in collision_query.iter_mut() {
        collider.collision.is_collided = false;
        collider.collision.collision_side.clear();
    }
}

pub fn internal_collision_detection_system<TType: Component>(
    mut colliders_query: Query<(Entity, &mut Collidable, &Transform), With<TType>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    if colliders_query.is_empty() {
        return;
    }

    let mut combinations = colliders_query.iter_combinations_mut();

    while let Some([mut first_tuple, mut second_tuple]) = combinations.fetch_next() {
        find_collisions(
            &mut first_tuple.1,
            &mut second_tuple.1,
            first_tuple.2,
            second_tuple.2,
            first_tuple.0,
            second_tuple.0,
            &mut collision_event_writer,
        )
    }
}

pub fn collision_detection_system<TFirst: Component, TSecond: Component>(
    mut first_query: Query<(Entity, &mut Collidable, &Transform), (With<TFirst>, Without<TSecond>)>,
    mut second_query: Query<
        (Entity, &mut Collidable, &Transform),
        (With<TSecond>, Without<TFirst>),
    >,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    if first_query.is_empty() || second_query.is_empty() {
        return;
    }

    for (first_entity, mut first_collide_options, first_transform) in first_query.iter_mut() {
        for (second_entity, mut second_collide_options, second_transform) in second_query.iter_mut()
        {
            find_collisions(
                first_collide_options.as_mut(),
                second_collide_options.as_mut(),
                first_transform,
                second_transform,
                first_entity,
                second_entity,
                &mut collision_event_writer,
            )
        }
    }
}

fn find_collisions(
    mut first_collidable: &mut Collidable,
    mut second_collidable: &mut Collidable,
    first_transform: &Transform,
    second_transform: &Transform,
    first_entity: Entity,
    second_entity: Entity,
    mut collision_event_writer: &mut EventWriter<CollisionEvent>,
) {
    let collision = collide(
        first_transform.translation,
        first_collidable.size,
        second_transform.translation,
        second_collidable.size,
    );

    if let Some(collision) = collision {
        resolve_collision(&mut first_collidable, &mut second_collidable, collision);
        collision_event_writer.send(CollisionEvent::new(first_entity, second_entity));
    }
}

fn resolve_collision(
    mut first_collidable: &mut Collidable,
    mut second_collidable: &mut Collidable,
    collision: Collision,
) {
    first_collidable.collision.is_collided = true;
    second_collidable.collision.is_collided = true;

    match collision {
        bevy::sprite::collide_aabb::Collision::Left => {
            second_collidable
                .collision
                .collision_side
                .push(Collision::Left);
            first_collidable
                .collision
                .collision_side
                .push(Collision::Right);
        }
        bevy::sprite::collide_aabb::Collision::Right => {
            second_collidable
                .collision
                .collision_side
                .push(Collision::Right);
            first_collidable
                .collision
                .collision_side
                .push(Collision::Left);
        }
        bevy::sprite::collide_aabb::Collision::Top => {
            second_collidable
                .collision
                .collision_side
                .push(Collision::Top);
            first_collidable
                .collision
                .collision_side
                .push(Collision::Bottom);
        }
        bevy::sprite::collide_aabb::Collision::Bottom => {
            second_collidable
                .collision
                .collision_side
                .push(Collision::Bottom);
            first_collidable
                .collision
                .collision_side
                .push(Collision::Top);
        }
        bevy::sprite::collide_aabb::Collision::Inside => {
            second_collidable
                .collision
                .collision_side
                .push(Collision::Inside);
            first_collidable
                .collision
                .collision_side
                .push(Collision::Inside);
        }
    }
}
