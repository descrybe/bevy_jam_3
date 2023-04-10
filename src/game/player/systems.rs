use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;

use super::events::ChooseModificationEvent;
use super::{PLAYER_HEALTH, PLAYER_SIZE, PLAYER_SPEED};
use crate::assets_cache::resources::AssetsCache;
use crate::events::GameOver;
use crate::game::collision::components::{Collidable, CollisionData, Solid};
use crate::game::health::components::HealthComponent;
use crate::game::health::events::DeathEvent;
use crate::game::score::resources::*;
use crate::game::GameSimulationState;
use crate::AppState;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

pub fn animate_player(
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

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_service: Res<AssetsCache>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();

    // Animation
    let texture_handle = asset_service.sprites.characters.wizzard.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(200.0, 200.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 4 };
    let sprite_index = animation_indices.first;

    let transformation = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: sprite_index,
                custom_size: Option::Some(Vec2::new(200.0, 200.0)),
                ..default()
            },
            transform: transformation,
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {
            experience: 0,
            level: 1,
            health: PLAYER_HEALTH,
        },
        HealthComponent::new(PLAYER_HEALTH),
        Collidable {
            size: Vec2 {
                x: PLAYER_SIZE * 0.9,
                y: PLAYER_SIZE * 0.9,
            },
            collision: CollisionData {
                is_collided: false,
                collision_side: Vec::new(),
            },
        },
        Solid {
            target_point: transformation.translation,
            collision_impact: 0.1,
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Solid), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((transform, mut solidity)) = player_query.get_single_mut() {
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

        solidity.target_point =
            transform.translation + direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn change_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut TextureAtlasSprite, With<Player>>,
) {
    let mut sprite = player_query.single_mut();

    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        sprite.flip_x = false;
    } else if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        sprite.flip_x = true;
    }
}

pub fn player_health_check_system(
    mut commands: Commands,
    mut event_reader: EventReader<DeathEvent>,
    mut game_over_event_writer: EventWriter<GameOver>,
    health_query: Query<&HealthComponent, With<Player>>,
    player_query: Query<Entity, With<Player>>,
    score: Res<Score>,
) {
    let health = health_query.get_single().unwrap();
    if event_reader.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        if !player_query.contains(event.entity) {
            continue;
        }

        if health.amount() < 0 {
            game_over_event_writer.send(GameOver { score: score.value });
            commands.entity(event.entity).despawn_recursive();
        }
    }
}

pub fn player_chose_modification(
    // player_query: Query<Entity, With<Player>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut event_reader: EventReader<ChooseModificationEvent>,
    mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>,
    // modification: Query<>
) {
    if event_reader.is_empty() {
        return;
    }

    for _event in event_reader.iter() {
        game_simulation_next_state.set(GameSimulationState::Running);
        app_state_next_state.set(AppState::Game);
    }
}
