use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use crate::game::random_position::screen_edge_position_generator::ScreenEdgePositionGenerator;
use crate::game::random_position::{Point, PositionGenerator, StraightLine};

use super::components::*;
use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut position_generator: ResMut<ScreenEdgePositionGenerator>,
) {
    let window = window_query.get_single().unwrap();

    let constraints = &mut StraightLine {
        first_point: Point { x: 0.0, y: 0.0 },
        second_point: Point {
            x: window.width(),
            y: window.height(),
        },
    };

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
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn change_enemy_direction(mut enemy_query: Query<(&mut Sprite, &Transform, &Enemy)>) {
    for (mut sprite, transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    // mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
    // player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    // let player_transform = player_query.single();

    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}
