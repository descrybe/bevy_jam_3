use crate::game::{health::components::HealthComponent, player::components::Player};

// use super::constants::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar {}

pub fn spawn_health_bar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/HPbar2.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(29.67, 24.67),
        3,
        3,
        Option::Some(Vec2::new(4.0, 10.0)),
        Option::Some(Vec2::new(5.0, 5.0)),
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    custom_size: Option::Some(Vec2::new(30.0, 15.0)),
                    ..default()
                },
                ..default()
            },
            HealthBar {},
        ))
        .with_children(|parent| {
            parent.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Option::Some(Vec2::new(30.0, 15.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-28.0, 0.0, 0.0),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 2,
                    custom_size: Option::Some(Vec2::new(30.0, 15.0)),
                    ..default()
                },
                transform: Transform::from_xyz(28.0, 0.0, 0.0),
                ..default()
            });
        });
}

pub fn stick_health_bar_to_player(
    mut health_bar_query: Query<&mut Transform, (Without<Player>, With<HealthBar>)>,
    health_query: Query<&HealthComponent, With<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let health = health_query.get_single().unwrap();
    println!("health.amount() {}", health.amount());

    if let Ok(mut health_bar_transform) = health_bar_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            health_bar_transform.translation.x = player_transform.translation.x;
            health_bar_transform.translation.y = player_transform.translation.y - 40.0;
        }
    };
}
