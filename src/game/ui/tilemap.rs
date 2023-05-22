use bevy::math::vec2;
use bevy::prelude::*;

use crate::game::player::components::Player;
use crate::{AppState, RESOLUTION};

pub const TILE_SIZE: f32 = 64.0;
const FIELD_SIZE: f32 = RESOLUTION.x / TILE_SIZE + 10.0;

#[derive(Component)]
pub struct Tile;

#[derive(Resource)]
pub struct TileField {
    size_in_tiles: Vec2,
    center_position_in_tiles: Vec2,
}

impl FromWorld for TileField {
    fn from_world(_: &mut World) -> Self {
        let width = RESOLUTION.x;
        let height = RESOLUTION.y;
        TileField {
            size_in_tiles: vec2(FIELD_SIZE, FIELD_SIZE),
            center_position_in_tiles: vec2(width / 2.0, height / 2.0),
        }
    }
}

impl TileField {
    pub fn center_position_in_tiles_mut(&mut self) -> &mut Vec2 {
        &mut self.center_position_in_tiles
    }

    pub fn center_position_in_tiles(&self) -> Vec2 {
        self.center_position_in_tiles
    }

    pub fn size_in_tiles(&self) -> Vec2 {
        self.size_in_tiles
    }
}

#[derive(Component)]
pub struct BackgroundTile;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundAssets>()
            .init_resource::<TileField>()
            .add_startup_system(load_background_assets)
            .add_system(move_background_frame.in_set(OnUpdate(AppState::Game)))
            .add_systems((delete_background, spawn_background).chain());
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

pub fn delete_background(mut commands: Commands, tiles: Query<Entity, With<BackgroundTile>>) {
    for tile in tiles.iter() {
        commands.entity(tile).despawn();
    }
}

pub fn move_background_frame(
    mut tile_field: ResMut<TileField>,
    player_position: Query<&Transform, With<Player>>,
) {
    let player_translation = player_position.get_single().unwrap().translation;

    tile_field.center_position_in_tiles_mut().x = translate_to_tiles(player_translation.x);
    tile_field.center_position_in_tiles_mut().y = translate_to_tiles(player_translation.y);
}

fn translate_to_tiles(source: f32) -> f32 {
    source / TILE_SIZE
}

pub fn spawn_background(
    mut commands: Commands,
    background_assets: Res<BackgroundAssets>,
    tile_field: Res<TileField>,
) {
    let width = tile_field.size_in_tiles().x as i32;
    let height = tile_field.size_in_tiles().y as i32;
    for x in -width..width {
        for y in -height..height {
            let transform = Transform::from_xyz(
                (x + (tile_field.center_position_in_tiles().x as i32)) as f32 * TILE_SIZE,
                (y + (tile_field.center_position_in_tiles().y as i32)) as f32 * TILE_SIZE,
                0.0,
            );

            commands.spawn((
                SpriteBundle {
                    transform,
                    texture: background_assets.tile.clone_weak(),
                    ..Default::default()
                },
                BackgroundTile {},
            ));
        }
    }
}
