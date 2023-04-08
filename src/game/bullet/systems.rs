use super::{components::Bullet, BULLET_DAMAGE};
use crate::game::{
    cooldown::CooldownState,
    cooldown::{components::CooldownComponent, events::CooldownEvent},
    damage::{components::DamageDealerComponent, events::DamageEvent},
    enemy::{components::Enemy, ENEMY_SIZE},
    player::components::Player,
    rotator::components::Rotator,
    target::components::{DirectionHolderComponent, TargetHolderComponent},
};

use bevy::{
    prelude::{
        AssetServer, Commands, Entity, EventWriter, Query, Res, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Timer},
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
    asset_service: Res<AssetServer>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    mut cooldown_event_writer: EventWriter<CooldownEvent>,
    cooldown_query: Query<&CooldownComponent, With<CooldownComponent>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    let nearest_entity = get_nearest_enity(enemy_query, &player_query);

    let bullet_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                    ..default()
                },
                transform: player_query.get_single().unwrap().clone(),
                texture: asset_service.load("sprites/projectile.png"),
                ..default()
            },
            Bullet {},
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
            CooldownComponent {
                seconds: 1,
                state: CooldownState::PAUSED,
                timer: Timer::from_seconds(1.0, bevy::time::TimerMode::Repeating),
            },
        ))
        .id();
    if let Ok(cooldown_component) = cooldown_query.get(bullet_entity) {
        match cooldown_component.state {
            CooldownState::READY => {
                cooldown_event_writer.send(CooldownEvent {
                    entity: bullet_entity,
                });
            }
            CooldownState::PAUSED => {
                return;
            }
        }
    } else {
        return;
    }
}

pub fn bullet_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<
        (
            Entity,
            &TargetHolderComponent,
            &Transform,
            &DamageDealerComponent,
        ),
        With<Bullet>,
    >,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_event_writer: EventWriter<DamageEvent>,
) {
    for (bullet_entity, bullet_target, bullet_transform, damage_dealer) in bullet_query.iter() {
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
            commands.entity(bullet_entity).despawn();
            damage_event_writer.send(DamageEvent {
                dealer: bullet_entity,
                target: enemy_entity,
            })
        }
    }
}
