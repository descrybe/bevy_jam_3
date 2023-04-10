use bevy::prelude::*;

use super::components::DiceButton;
use crate::game::{
    player::{
        components::Player,
        events::{ChooseModificationEvent, Modification},
    },
    ui::dices_preview::DICE_DIMENTION_SPRITE_SIZE,
};

pub const CHOOSE_DICE_ENTITY_SIZE: f32 = 150.0;

pub fn spawn_lvlup_dices(
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_transform = player_query.get_single().unwrap();

    let texture_handle = asset_server.load("sprites/dice_sides.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(DICE_DIMENTION_SPRITE_SIZE, DICE_DIMENTION_SPRITE_SIZE),
        2,
        3,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for _ in 0..3 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    custom_size: Option::Some(Vec2::new(
                        CHOOSE_DICE_ENTITY_SIZE,
                        CHOOSE_DICE_ENTITY_SIZE,
                    )),
                    ..default()
                },
                transform: Transform::from_xyz(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    100.0,
                ),
                ..default()
            },
            DiceButton {},
        ));
    }
}

pub fn lvlup_dice_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<DiceButton>)>,
    mut modification_choose_event: EventWriter<ChooseModificationEvent>,
    // modification: Query<>
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                // TODO: fixed from state to events
                modification_choose_event.send(ChooseModificationEvent {
                    modification: Modification::AutoAttack,
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn despawn_lvlup_dices(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut dices_query: Query<, >
) {
    println!("despawn!@");
    // if let Ok(main_menu_entity) = main_menu_query.get_single() {
    //     commands.entity(main_menu_entity).despawn_recursive();
    // }
}
