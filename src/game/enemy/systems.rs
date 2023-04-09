use std::ops::{Add, Sub};

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use crate::game::damage::components::DamageDealerComponent;
use crate::game::damage::events::DamageEvent;
use crate::game::health::components::HealthComponent;
use crate::game::health::events::DeathEvent;
use crate::game::player::components::Player;
use crate::game::player::PLAYER_SIZE;
use crate::game::random_position::screen_edge_position_generator::ScreenEdgePositionGenerator;
use crate::game::random_position::{Point, PositionGenerator, StraightLine};
use crate::game::target::components::{DirectionHolderComponent, TargetHolderComponent};

use super::events::WaveSpawnEvent;
use super::resources::EnemyWavesSpawnConfig;
use super::{components::*, ENEMY_DAMAGE, ENEMY_HEALTH};
use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

pub fn animate_enemy(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn spawn_enemie_wave(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    mut position_generator: ResMut<ScreenEdgePositionGenerator>,
    mut wave_spawn_event: EventReader<WaveSpawnEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    // Animation
    let texture_handle = asset_server.load("sprites/sombiespritemap.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    //
    for _ in 0..ENEMY_COUNT {
        // Animation
        let animation_indices = AnimationIndices { first: 1, last: 4 };
        let sprite_index = animation_indices.first;
        //
        let position = position_generator.generate(constraints).unwrap();
        let base_position = Vec3::from([position.x, position.y, 0.0]);
        let translated_position = base_position
            .add(player_transform.translation)
            .sub(centrolization_vector);
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: sprite_index,
                    custom_size: Option::Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    flip_x: true,
                    ..default()
                },
                transform: Transform::from_translation(translated_position),
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
            // Animation
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &DirectionHolderComponent), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, direction_holder) in enemy_query.iter_mut() {
        let direction = Vec3::new(
            direction_holder.direction.x,
            direction_holder.direction.y,
            0.0,
        );

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
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

pub fn enemy_hit_player(
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(&Transform, &DamageDealerComponent), With<Enemy>>,
    mut damage_event_writer: EventWriter<DamageEvent>,
) {
    if enemy_query.is_empty() || player_query.is_empty() {
        return;
    }

    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for (enemy_transform, damage_dealer) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                damage_event_writer.send(DamageEvent {
                    damage_amount: damage_dealer.damage,
                    target: player_entity,
                });
            }
        }
    }
}
