use bevy::prelude::*;
use bevy::render::render_resource::SamplerDescriptor;
use bevy::render::texture::ImageSampler;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub const TILE_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Tile;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundAssets>()
            .add_startup_systems((load_background_assets, spawn_background).chain())
            .add_system(repeat_background);
    }
}

#[derive(Debug, Default, Resource)]
pub struct BackgroundAssets {
    tile: Handle<Image>,
}

fn load_background_assets(
    asset_server: Res<AssetServer>,
    mut background_assets: ResMut<BackgroundAssets>,
) {
    background_assets.tile = asset_server.load("sprites/tileset.png");
}

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

pub fn repeat_background(
    background_assets: Res<BackgroundAssets>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(mut image) = images.get_mut(&background_assets.tile) {
        image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
            address_mode_u: bevy::render::render_resource::AddressMode::Repeat,
            address_mode_v: bevy::render::render_resource::AddressMode::Repeat,
            address_mode_w: bevy::render::render_resource::AddressMode::Repeat,
            ..Default::default()
        });
    }
}

pub fn spawn_background(mut commands: Commands, background_assets: Res<BackgroundAssets>) {
    for x in -20..20 {
        for y in -20..20 {
            let transform = Transform::from_xyz(x as f32 * TILE_SIZE * 5.0, y as f32 * TILE_SIZE * 5.0, 0.0)
                .with_scale(Vec3::splat(10.0));
            commands.spawn(SpriteBundle {
                transform,
                texture: background_assets.tile.clone_weak(),
                ..Default::default()
            });
        }
    }
}
