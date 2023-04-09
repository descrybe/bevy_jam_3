use super::{
    components::{Bullet, BulletAbility},
    events::LaunchBulletEvent,
    BULLET_DAMAGE, BULLET_SIZE, BULLET_SPEED, COOLDOWN_DELAY, ROTATION_SPEED,
};
use crate::{
    assets_cache::resources::AssetsCache,
    game::{
        ability::{
            components::{AbilityComponent, Cooldown, PeriodicAbility},
            events::TriggerAbilityEvent,
        },
        collision::components::{Collidable, CollisionData},
        damage::components::{DamageDealerComponent, SelfDestructable},
        enemy::components::Enemy,
        flight::components::Flight,
        player::components::Player,
        rotator::components::Rotator,
        target::components::{DirectionHolderComponent, TargetHolderComponent},
    },
};

use bevy::{
    prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

fn get_nearest_entity(
    enemy_query: &Query<(Entity, &Transform), With<Enemy>>,
    source_transform: &Transform,
) -> Entity {
    let source_translation = source_transform.translation;

    let enemy_distance = enemy_query
        .iter()
        .map(|(_, enemy)| source_translation.distance(enemy.translation) as i32)
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

pub fn generate_ability_entity(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player_entity = player_query.get_single().unwrap();

    commands.spawn((
        AbilityComponent {
            owner: player_entity,
        },
        Cooldown::new(COOLDOWN_DELAY),
        PeriodicAbility {},
        BulletAbility {},
    ));
}

pub fn trigger_bullet_ability(
    mut event_reader: EventReader<TriggerAbilityEvent>,
    mut event_writer: EventWriter<LaunchBulletEvent>,
    ability_query: Query<&AbilityComponent, With<BulletAbility>>,
    common_query: Query<&Transform>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    if event_reader.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        if !ability_query.contains(event.owner()) || enemy_query.is_empty() {
            continue;
        }

        let ability_options = ability_query.get(event.owner()).unwrap();

        let owner_transform = common_query.get(ability_options.owner).unwrap();
        let target = get_nearest_entity(&enemy_query, owner_transform);

        event_writer.send(LaunchBulletEvent {
            owner: ability_options.owner,
            target: target,
        })
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    asset_service: Res<AssetsCache>,
    common_query: Query<&Transform>,
    mut event_reader: EventReader<LaunchBulletEvent>,
) {
    if event_reader.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        if !common_query.contains(event.owner) || !common_query.contains(event.target) {
            return;
        }
        let source = common_query.get(event.owner).unwrap();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                    ..default()
                },
                transform: source.clone(),
                texture: asset_service.sprites.projectiles.bottle.clone(),
                ..default()
            },
            Bullet {},
            Flight {
                speed: BULLET_SPEED,
            },
            TargetHolderComponent {
                target_entity: event.target,
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
            SelfDestructable::new(0.1),
            Collidable {
                size: Vec2 {
                    x: BULLET_SIZE,
                    y: BULLET_SIZE,
                },
                collision: CollisionData {
                    is_collided: false,
                    collision_side: Vec::new(),
                },
            },
        ));
    }
}
