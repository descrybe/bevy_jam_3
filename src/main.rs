use ::bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

mod services;
pub const PLAYER_SIZE: f32 = 50.0;
pub const PLAYER_SPEED: f32 = 700.0;
pub const ENEMY_SIZE: f32 = 60.0;
pub const ENEMY_COUNT: u32 = 10;
pub const ENEMY_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(camera_follow)
        .add_system(change_player_direction)
        .add_system(change_enemy_direction)
        .add_event::<GameOver>()
        .init_resource::<Score>()
        .add_system(game_over_hander)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct MainCamera {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

pub struct GameOver {
    pub score: u32,
}

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

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        MainCamera {},
    ));
}

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

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<MainCamera>)>,
) {
    let mut camera = camera_query.single_mut();
    let player = player_query.single();

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
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

pub fn change_enemy_direction(mut enemy_query: Query<(&mut Sprite, &Transform, &Enemy)>) {
    for (mut sprite, transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        println!("direction {}", enemy.direction.x);
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();

    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(0.0, 0.0, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                // commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn game_over_hander(mut game_over_event_writer: EventReader<GameOver>) {
    for event in game_over_event_writer.iter() {
        println!("Game over!");
    }
}
