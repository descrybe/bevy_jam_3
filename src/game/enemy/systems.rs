use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::random;

use super::components::*;
use super::{ENEMY_COUNT, ENEMY_SIZE, ENEMY_SPEED};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..ENEMY_COUNT {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2::new(ENEMY_SIZE, ENEMY_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(random_x, random_y, 0.0),
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
