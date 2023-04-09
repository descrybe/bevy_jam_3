use super::{components::Bullet, BULLET_DAMAGE};
use crate::{
    assets_cache::resources::AssetsCache,
    game::{
        collision::components::{Collidable, CollisionData},
        damage::{components::DamageDealerComponent, events::DamageEvent},
        enemy::components::Enemy,
        flight::{components::Flight, resources::FireSpawnConfig},
        player::components::Player,
        rotator::components::Rotator,
        target::components::{DirectionHolderComponent, TargetHolderComponent},
    },
};

use bevy::{
    prelude::{Commands, Entity, EventReader, Query, Res, ResMut, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    utils::default,
};

const BULLET_SIZE: f32 = 40.0;
const ROTATION_SPEED: f32 = 720.0;
const BULLET_SPEED: f32 = 280.0;

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
    asset_service: Res<AssetsCache>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
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
            texture: asset_service.sprites.projectiles.bottle.clone(),
            ..default()
        },
        Bullet {},
        Flight {
            speed: BULLET_SPEED,
        },
        TargetHolderComponent {
            target_entity: nearest_entity,
        },
        DirectionHolderComponent {
            direction: Vec2 { x: 0.0, y: 0.0 },
        },
        Rotator {
            angle: ROTATION_SPEED,
        },
        DamageDealerComponent {
            damage: BULLET_DAMAGE,
        },
        Collidable {
            size: Vec2 {
                x: BULLET_SIZE,
                y: BULLET_SIZE,
            },
            is_solid: false,
            collision: CollisionData {
                is_collided: false,
                collision_side: Vec::new(),
            },
        },
    ));
}

pub fn bullet_damage_event_handler(
    mut commands: Commands,
    mut event_reader: EventReader<DamageEvent>,
    query: Query<&Bullet>,
) {
    if event_reader.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        let entity = event.dealer;
        if !query.contains(entity) {
            continue;
        }

        commands.entity(entity).despawn();
    }
}
