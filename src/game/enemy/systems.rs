use std::ops::{Add, Sub};

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use crate::game::player::components::Player;
use crate::game::random_position::screen_edge_position_generator::ScreenEdgePositionGenerator;
use crate::game::random_position::{Point, PositionGenerator, StraightLine};
use crate::game::target::components::{DirectionHolderComponent, TargetHolderComponent};

use super::components::*;
use super::events::WaveSpawnEvent;
use super::resources::EnemyWavesSpawnConfig;
use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};

pub fn spawn_enemie_wave(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
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
                texture: asset_server.load("sprites/zombie.png"),
                ..default()
            },
            Enemy {},
            TargetHolderComponent {
                target_entity: player_entity,
            },
            DirectionHolderComponent {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
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

    println!("Zombies approaching");

    wave_event.send(WaveSpawnEvent {});
}
