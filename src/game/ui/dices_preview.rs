use crate::game::player::components::Player;

use bevy::{prelude::*, window::PrimaryWindow};

pub const DICE_DIMENTION_SPRITE_SIZE: f32 = 202.0;
pub const DICE_DIMENTION_ENTITY_SIZE: f32 = 45.0;

#[derive(Component)]
pub struct FirstDicePreview {}

#[derive(Component)]
pub struct SecondDicePreview {}

pub fn spawn_preview_dices(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/dice_sides.png");
    let texture_handle_second = asset_server.load("sprites/dice_sides.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(DICE_DIMENTION_SPRITE_SIZE, DICE_DIMENTION_SPRITE_SIZE),
        2,
        3,
        None,
        None,
    );
    let texture_atlas_second = TextureAtlas::from_grid(
        texture_handle_second,
        Vec2::new(DICE_DIMENTION_SPRITE_SIZE, DICE_DIMENTION_SPRITE_SIZE),
        2,
        3,
        None,
        None,
    );

    let texture_atlas_handle_first = texture_atlases.add(texture_atlas);
    let texture_atlas_handle_second = texture_atlases.add(texture_atlas_second);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_first.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Option::Some(Vec2::new(
                    DICE_DIMENTION_ENTITY_SIZE,
                    DICE_DIMENTION_ENTITY_SIZE,
                )),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        },
        FirstDicePreview {},
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle_second.clone(),
            sprite: TextureAtlasSprite {
                index: 1,
                custom_size: Option::Some(Vec2::new(
                    DICE_DIMENTION_ENTITY_SIZE,
                    DICE_DIMENTION_ENTITY_SIZE,
                )),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..default()
        },
        SecondDicePreview {},
    ));
}

pub fn stick_first_dice(
    mut first_dice_query: Query<&mut Transform, (Without<Player>, With<FirstDicePreview>)>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let mut first_dice_transform = first_dice_query.get_single_mut().unwrap();

    if let Ok(player_transform) = player_query.get_single() {
        first_dice_transform.translation.x = player_transform.translation.x - 30.0;
        first_dice_transform.translation.y = player_transform.translation.y - window.height() / 2.0 + 100.0;
    }
}

pub fn stick_second_dice(
    mut second_dice_query: Query<&mut Transform, (Without<Player>, With<SecondDicePreview>)>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let mut second_dice_transform = second_dice_query.get_single_mut().unwrap();

    if let Ok(player_transform) = player_query.get_single() {
        second_dice_transform.translation.x = player_transform.translation.x + 30.0;
        second_dice_transform.translation.y = player_transform.translation.y - window.height() / 2.0 + 100.0;
    }
}
