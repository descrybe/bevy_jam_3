use crate::game::{enemy::ENEMY_SIZE, player::components::Player};

use super::constants::*;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::seq::index;

#[derive(Component)]
pub struct ExperienceBar {}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

pub fn animate_sprite(
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

pub fn setup_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/sombiespritemap.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 4 };
    let sprite_index = animation_indices.first;
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: sprite_index,
                custom_size: Option::Some(Vec2::new(ENEMY_SIZE / 4.0, ENEMY_SIZE / 4.0)),
                ..default()
            },
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

// pub fn spawn_exp_bar(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     texture_assets: Assets<TextureAtlas>,
// ) {
//     commands
//         .spawn(NodeBundle {
//             style: EXPERIENCE_BAR_WRAPPER_STYLES,
//             ..default()
//         })
//         .with_children(|parent| {
//             parent
//                 .spawn(NodeBundle {
//                     style: Style {
//                         size: Size::width(EXPERIENCE_BAR_WIDTH),
//                         border: UiRect::all(Val::Px(2.0)),
//                         ..default()
//                     },
//                     background_color: EXPERIENCE_BAR_BORDER_COLOR.into(),
//                     ..default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn(NodeBundle {
//                         style: Style {
//                             size: Size::width(Val::Percent(100.0)),
//                             ..default()
//                         },
//                         background_color: EXPREIENCE_BAR_BACKGROUND_COLOR.into(),
//                         ..default()
//                     });
//                 })
//                 .with_children(|parent| {
//                     parent
//                         .spawn(NodeBundle {
//                             style: Style {
//                                 size: Size {
//                                     width: Val::Percent(75.0), // сюда передавать Experience.value/EXPERIENCE_TO_LVL_UP
//                                     height: Val::Px(31.0),
//                                 },
//                                 position_type: PositionType::Absolute,
//                                 position: UiRect {
//                                     top: Val::Auto,
//                                     left: Val::Undefined,
//                                     right: Val::Auto,
//                                     bottom: Val::Undefined,
//                                 },
//                                 ..default()
//                             },
//                             background_color: EXPREIENCE_BAR_MAIN_COLOR.into(),
//                             ..default()
//                         })
//                         .with_children(|parent| {
//                             parent.spawn((
//                                 TextBundle::from_section(
//                                     "LVL 1", // тут вставлять значение текущего уровня
//                                     TextStyle {
//                                         font: asset_server.load("fonts/CyrillicPixel.ttf"),
//                                         font_size: 30.0,
//                                         color: Color::WHITE,
//                                     },
//                                 )
//                                 .with_style(Style {
//                                     position_type: PositionType::Absolute,
//                                     position: UiRect {
//                                         top: Val::Auto,
//                                         left: Val::Auto,
//                                         right: Val::Auto,
//                                         bottom: Val::Px(20.0),
//                                     },
//                                     ..default()
//                                 }),
//                                 Label,
//                             ));
//                         });
//                 });
//         });
// }

pub fn spawn_exp_bar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/XPbar.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 24.6), 3, 3, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 4,
                    custom_size: Option::Some(Vec2::new(100.0, 40.0)),
                    ..default()
                },
                ..default()
            },
            ExperienceBar {},
        ))
        .with_children(|parent| {
            parent.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Option::Some(Vec2::new(50.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(-200.0, 0.0, 0.0),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 5,
                    custom_size: Option::Some(Vec2::new(50.0, 40.0)),
                    ..default()
                },
                transform: Transform::from_xyz(200.0, 0.0, 0.0),
                ..default()
            });
        });
}

pub fn stick_exp_bar(
    mut xp_bar_query: Query<&mut Transform, (Without<Player>, With<ExperienceBar>)>,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let mut xp_bar_transform = xp_bar_query.get_single_mut().unwrap();
    let window  = window_query.get_single().unwrap();

    if let Ok(player_transform) = player_query.get_single() {
        xp_bar_transform.translation.x = player_transform.translation.x;
        xp_bar_transform.translation.y = player_transform.translation.y - window.height() / 2.0 + 50.0;
    }
}
