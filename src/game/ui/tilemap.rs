use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::AppState;

pub const TILE_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Tile;

pub fn spawn_simple_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file = File::open("assets/map.txt").expect("No map file found");
    let image_handle = asset_server.load("sprites/tileset.png");

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, _char) in line.chars().enumerate() {

                // let tile = spawn_sprite(
                //     &mut commands,
                //     char as usize,
                //     Color::rgb(0.9, 0.9, 0.9),
                //     Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 0.0),
                //     &asset_server,
                // );

                // commands.spawn(SpriteSheetBundle {
                //     transform: Transform::from_xyz(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, -1.0),
                //     sprite: TextureAtlasSprite {
                //         custom_size: Some(Vec2::splat(TILE_SIZE * 2.0)),
                //         index: _char as usize,
                //         ..default()
                //     },
                //     texture_atlas: image_handle.clone(),
                //     ..default()
                // });

                commands.spawn(SpriteBundle {
                    transform: Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 1.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(TILE_SIZE * 2.0)),
                        ..default()
                    },
                    texture: image_handle.clone(),
                    ..default()
                });
            }
        }
    }
}

// TODO: refactor this to plugin and fix function to current version
pub fn spawn_sprite(
    commands: &mut Commands,
    index: usize,
    color: Color,
    translation: Vec3,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let image_handle = asset_server.load("sprites/tileset.png");

    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    commands
        .spawn((
            SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: image_handle.clone(),
                transform: Transform {
                    translation: translation,
                    ..Default::default()
                },
                ..Default::default()
            },
            Tile,
        ))
        .id()
}
