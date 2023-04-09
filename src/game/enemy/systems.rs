use std::ops::{Add, Sub};

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use crate::assets_cache::resources::AssetsCache;
use crate::game::collision::components::{Collidable, CollisionData, Solid};
use crate::game::damage::components::DamageDealerComponent;
use crate::game::health::components::HealthComponent;
use crate::game::health::events::DeathEvent;
use crate::game::player::components::Player;
use crate::game::random_position::screen_edge_position_generator::ScreenEdgePositionGenerator;
use crate::game::random_position::{Point, PositionGenerator, StraightLine};
use crate::game::target::components::{DirectionHolderComponent, TargetHolderComponent};

use super::events::WaveSpawnEvent;
use super::resources::EnemyWavesSpawnConfig;
use super::{components::*, ENEMY_DAMAGE, ENEMY_HEALTH};
use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};

pub fn spawn_enemie_wave(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetsCache>,
    mut position_generator: ResMut<ScreenEdgePositionGenerator>,
    mut wave_spawn_event: EventReader<WaveSpawnEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    if wave_spawn_event.is_empty() {
        return;
    }
    wave_spawn_event.clear();

    let window = window_query.get_single().unwrap();

    let constraints = &mut StraightLine {
        first_point: Point { x: 0.0, y: 0.0 },
        second_point: Point {
            x: window.width(),
            y: window.height(),
        },
    };

    let (player_entity, player_transform) = player_query.get_single().unwrap();

    let centrolization_vector = Vec3::from([window.width() / 2.0, window.height() / 2.0, 0.0]);
    for _ in 0..ENEMY_COUNT {
        let position = position_generator.generate(constraints).unwrap();
        let base_position = Vec3::from([position.x, position.y, 0.0]);
        let translated_position = base_position
            .add(player_transform.translation)
            .sub(centrolization_vector);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(translated_position),
                texture: asset_server.sprites.characters.zombie.clone(),
                ..default()
            },
            Enemy {},
            TargetHolderComponent {
                target_entity: player_entity,
            },
            DirectionHolderComponent {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
            HealthComponent::new(ENEMY_HEALTH),
            DamageDealerComponent {
                damage: ENEMY_DAMAGE,
            },
            Collidable {
                size: Vec2 {
                    x: ENEMY_SIZE * 0.6,
                    y: ENEMY_SIZE * 0.6,
                },
                collision: CollisionData {
                    is_collided: false,
                    collision_side: Vec::new(),
                },
            },
            Solid {
                target_point: translated_position,
                collision_impact: 0.4,
            },
        ));
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&Transform, &DirectionHolderComponent, &mut Solid), With<Enemy>>,
    time: Res<Time>,
) {
    for (transform, direction_holder, mut solidity) in enemy_query.iter_mut() {
        let direction = Vec3::new(
            direction_holder.direction.x,
            direction_holder.direction.y,
            0.0,
        );
        solidity.target_point =
            transform.translation + direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn wave_timer_tracking_system(
    mut config: ResMut<EnemyWavesSpawnConfig>,
    time: Res<Time>,
    mut wave_event: EventWriter<WaveSpawnEvent>,
) {
    config.timer.tick(time.delta());

    if !config.timer.finished() {
        return;
    }

    wave_event.send(WaveSpawnEvent {});
}

pub fn kill_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    mut death_event_reader: EventReader<DeathEvent>,
) {
    if death_event_reader.is_empty() {
        return;
    }

    for event in death_event_reader.iter() {
        if !enemy_query.contains(event.entity) {
            continue;
        }

        commands.entity(event.entity).despawn();
    }
}
