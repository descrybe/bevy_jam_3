use super::{
    components::{Radiance, RadianceAbility},
    events::LaunchRadianceEvent,
    COOLDOWN_DELAY, RADIANCE_DAMAGE, RADIANCE_SIZE,
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
        player::components::Player,
        player_binder::components::PlayerBinder,
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
        RadianceAbility {},
    ));
}

pub fn trigger_bullet_ability(
    mut event_reader: EventReader<TriggerAbilityEvent>,
    mut event_writer: EventWriter<LaunchRadianceEvent>,
    ability_query: Query<&AbilityComponent, With<RadianceAbility>>,
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

        event_writer.send(LaunchRadianceEvent {
            owner: ability_options.owner,
            target: target,
        })
    }
}

pub fn spawn_radiance(
    mut commands: Commands,
    asset_service: Res<AssetsCache>,
    common_query: Query<&Transform>,
    mut event_reader: EventReader<LaunchRadianceEvent>,
    radiance_query: Query<&Transform, With<Radiance>>,
    radiance_ability_query: Query<&Transform, With<RadianceAbility>>,
) {
    if event_reader.is_empty() || !radiance_query.is_empty() || !radiance_ability_query.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        if !common_query.contains(event.owner) || !common_query.contains(event.target) {
            return;
        }
        let source = common_query.get(event.owner).unwrap();
        let mut source_copy = source.clone();
        source_copy.translation.z -= 1.0;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2::new(RADIANCE_SIZE, RADIANCE_SIZE)),
                    ..default()
                },
                transform: source_copy,
                texture: asset_service.sprites.projectiles.bottle.clone(),
                ..default()
            },
            Radiance {},
            PlayerBinder {},
            TargetHolderComponent {
                target_entity: event.target,
            },
            DirectionHolderComponent {
                direction: Vec2 { x: 0.0, y: 0.0 },
            },
            DamageDealerComponent {
                damage: RADIANCE_DAMAGE,
            },
            SelfDestructable::new(0.1),
            Collidable {
                size: Vec2 {
                    x: RADIANCE_SIZE,
                    y: RADIANCE_SIZE,
                },
                is_solid: false,
                collision: CollisionData {
                    is_collided: false,
                    collision_side: Vec::new(),
                },
            },
        ));
    }
}
