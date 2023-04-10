use std::ops::{Add, Sub};

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use crate::AppState;
use crate::assets_cache::resources::AssetsCache;
use crate::game::GameSimulationState;
use crate::game::collision::components::{Collidable, CollisionData, Solid};
use crate::game::damage::components::DamageDealerComponent;
use crate::game::health::components::HealthComponent;
use crate::game::health::events::DeathEvent;
use crate::game::player::components::{Player, EXPERIENCE_THRESHOLD};
use crate::game::random_position::screen_edge_position_generator::ScreenEdgePositionGenerator;
use crate::game::random_position::{Point, PositionGenerator, StraightLine};
use crate::game::target::components::{DirectionHolderComponent, TargetHolderComponent};

use super::events::WaveSpawnEvent;
use super::resources::EnemyWavesSpawnConfig;
use super::{components::*, ENEMY_DAMAGE, ENEMY_HEALTH};
use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED, DEFAULT_EXPERINCE_DROP_VALUE};

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
    // TODO: исправить импорт на ассет кеш
    asset_server: Res<AssetServer>,
    // asset_server: Res<AssetsCache>,
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
    enemy_query: Query<Entity, With<Enemy>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut death_event_reader: EventReader<DeathEvent>,
    mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>,
    mut player_query: Query<&mut Player>,
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

    if let Ok(mut player) = player_query.get_single_mut() {
        player.give_exp(DEFAULT_EXPERINCE_DROP_VALUE);

        // TODO: add proper level scaling
        if player.get_level() > 4 {
            player.give_exp(20);
        }

        // TODO: refactor to event(for example: LvlUpEvent)
        if player.get_experience_amount() >= EXPERIENCE_THRESHOLD {
            player.lvl_up(app_state_next_state, game_simulation_next_state);
        }
    }
}
