use std::{ops::Sub};

use bevy::{
    prelude::{AssetServer, Commands, Input, KeyCode, Query, Res, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
    utils::default,
    window::{PrimaryWindow, Window},
};
use crate::components::{bullet::Bullet, flight::Flight, enemy::{Enemy}, player::Player};

pub(crate) const BULLET_SIZE: f32 = 50.0;

fn get_direction(
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>
) -> Option<Vec2> {
    let player_translation = player_query.get_single().ok()?.translation;
    let enemy_distance =
        enemy_query.iter().map(|enemy| player_translation.distance(enemy.translation) as i32).collect::<Vec<i32>>();
    let min_distance = enemy_distance.iter().min();
    let min_index = enemy_distance.iter().position(|x| x == min_distance.unwrap())?;
    let nearest_enemy = enemy_query.iter().collect::<Vec<&Transform>>()[min_index];
    let direction_vec3 = nearest_enemy.translation.sub(player_translation).normalize();
    let direction = Vec2{x: direction_vec3.x, y: direction_vec3.y};

    return Some(direction);
}

pub fn spawn_bullet(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_service: Res<AssetServer>,
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<&Transform, With<Player>>
) {
    let bullet_key = keyboard_input.any_pressed([KeyCode::E]);

    if !bullet_key {
        return;
    }

    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Option::Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                ..default()
            },
            transform: player_query.get_single().unwrap().clone(),
            texture: asset_service.load("sprites/ball_blue_small.png"),
            ..default()
        },
        Bullet {}, Flight {speed: 100.0, direction: get_direction(enemy_query, player_query).unwrap()}
    ));
}
