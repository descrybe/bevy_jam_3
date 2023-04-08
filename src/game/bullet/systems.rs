use super::components::Bullet;
use crate::game::{
    enemy::{components::Enemy, ENEMY_SIZE},
    flight::{components::Flight, resources::FireSpawnConfig},
    player::components::Player, target::components::{TargetHolderComponent, DirectionHolderComponent},
};

// use std::ops::Sub;

use bevy::{
    prelude::{AssetServer, Commands, Entity, Query, Res, ResMut, Transform, Vec2, Vec3, With},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    utils::default,
};

const BULLET_SIZE: f32 = 25.0;

fn get_player_translation(player_query: Query<&Transform, With<Player>>) -> Option<Vec3> {
    return Some(player_query.get_single().ok()?.translation);
}

fn get_nearest_enity(
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: &Query<&Transform, With<Player>>,
) -> Entity {
    let player_translation = player_query.get_single().unwrap().translation;

    let enemy_distance = enemy_query
        .iter()
        .map(|(_, enemy)| player_translation.distance(enemy.translation) as i32)
        .collect::<Vec<i32>>();

    let min_distance = enemy_distance.iter().min();

    let min_index = enemy_distance
        .iter()
        .position(|x| x == min_distance.unwrap())
        .unwrap();

    let (min_enemy_entity, _) =
        enemy_query.iter().collect::<Vec<(Entity, &Transform)>>()[min_index];


    return min_enemy_entity;
}

pub fn spawn_bullet(
    mut commands: Commands,
    asset_service: Res<AssetServer>,
    enemy_query: Query<(Entity,&Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    mut config: ResMut<FireSpawnConfig>,
    time: Res<Time>,
) {
    config.timer.tick(time.delta());

    if !config.timer.finished() {
        return;
    }
    if enemy_query.is_empty() {
        return;
    }

    let nearest_entity = get_nearest_enity(enemy_query, &player_query);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Option::Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                ..default()
            },
            transform: player_query.get_single().unwrap().clone(),
            texture: asset_service.load("sprites/ball_blue_small.png"),
            ..default()
        },
        Bullet {},
        Flight {
            speed: 800.0,
        },
        TargetHolderComponent {
            target_entity: nearest_entity,
        },
        DirectionHolderComponent {
            direction: Vec2 { x: 0.0, y: 0.0 },
        }
    ));
}

pub fn bullet_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<(Entity,&TargetHolderComponent, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet_target, bullet_transform) in bullet_query.iter() {
        if !enemy_query.contains(bullet_target.target_entity) {
            commands.entity(bullet_entity).despawn();
            continue;
        }

        let (enemy_entity, nearest_enemy_transform) =
            enemy_query.get(bullet_target.target_entity).unwrap();
        let distance = nearest_enemy_transform
            .translation
            .distance(bullet_transform.translation);
        let player_radius = BULLET_SIZE / 2.0;
        let enemy_radius = ENEMY_SIZE / 2.0;

        if distance < player_radius + enemy_radius {
            commands.entity(enemy_entity).despawn();
            commands.entity(bullet_entity).despawn();
        }
    }
}