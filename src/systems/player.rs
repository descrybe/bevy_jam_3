use bevy::{
    prelude::{AssetServer, Commands, Input, KeyCode, Query, Res, Transform, Vec2, Vec3, With},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    utils::default,
    window::{PrimaryWindow, Window},
};

use crate::components::player::Player;

pub(crate) const PLAYER_SIZE: f32 = 50.0;
const PLAYER_SPEED: f32 = 700.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_service: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Option::Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_service.load("sprites/player_sprite.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let left_direction = keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]);
        let right_direction = keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]);
        let bottom_direction = keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]);
        let top_direction = keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]);

        if left_direction {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if right_direction {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if bottom_direction {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if top_direction {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn change_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Sprite, With<Player>>,
) {
    let mut sprite = player_query.single_mut();

    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        sprite.flip_x = true;
    } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        sprite.flip_x = false;
    }
}