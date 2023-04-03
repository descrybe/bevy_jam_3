use::bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 600.0;
pub const ENEMY_SIZE: f32 = 32.0;
pub const ENEMY_COUNT: u32 = 100;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(player_movement)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        .add_startup_system(spawn_camera)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_service: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_service.load("sprites/ball_blue_small.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..ENEMY_COUNT {
        let x: f32 = rng.gen::<f32>() * window.width();
        let y: f32 = rng.gen::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_small.png"),
                ..default()
            },
            Enemy {}
        ));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let left_direction =
            keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
        let right_direction =
            keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);
        let bottom_direction =
            keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S);
        let top_direction =
            keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W);

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