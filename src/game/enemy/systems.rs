use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use crate::game::player::components::Player;
use crate::game::random_position::screen_edge_position_generator::ScreenEdgePositionGenerator;
use crate::game::random_position::{Point, PositionGenerator, StraightLine};
use crate::game::target::components::{DirectionHolderComponent, TargetHolderComponent};

use super::components::*;
use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};

// TODO: Remove this
static mut SPAWNED: bool = false;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    mut position_generator: ResMut<ScreenEdgePositionGenerator>,
) {
    if unsafe { SPAWNED } {
        return;
    }

    let window = window_query.get_single().unwrap();

    let constraints = &mut StraightLine {
        first_point: Point { x: 0.0, y: 0.0 },
        second_point: Point {
            x: window.width(),
            y: window.height(),
        },
    };

    let player_entity = player_query.get_single().unwrap();

    for _ in 0..ENEMY_COUNT {
        let position = position_generator.generate(constraints).unwrap();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 0.0),
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

    unsafe {
        SPAWNED = true;
    }
}

pub fn change_enemy_direction(
    mut enemy_query: Query<(&mut Sprite, &Transform, &DirectionHolderComponent), With<Enemy>>,
) {
    for (mut sprite, transform, direction_holder) in enemy_query.iter_mut() {
        let direction = Vec3::new(
            direction_holder.direction.x,
            direction_holder.direction.y,
            0.0,
        );
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
