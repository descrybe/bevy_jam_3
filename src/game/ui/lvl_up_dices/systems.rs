use bevy::prelude::*;

use super::components::DiceButton;
use crate::{
    game::{
        player::{
            components::Player,
            events::{ChooseModificationEvent, Modification},
        },
        ui::dices_preview::DICE_DIMENTION_SPRITE_SIZE,
        GameSimulationState,
    },
    AppState,
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
    // let texture_atlas = TextureAtlas::from_grid(
    //     texture_handle,
    //     Vec2::new(DICE_DIMENTION_SPRITE_SIZE, DICE_DIMENTION_SPRITE_SIZE),
    //     2,
    //     3,
    //     None,
    //     None,
    // );
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let font = asset_server.load("fonts/CyrillicPixel.ttf");
    let text_alignment = TextAlignment::Center;

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        color: Color::WHITE,
    };

    commands.spawn(Text2dBundle {
        text: Text::from_section("Choose modification!", text_style.clone())
            .with_alignment(text_alignment),
        transform: Transform::from_xyz(
            player_transform.translation.x,
            player_transform.translation.y + 130.0,
            10.0,
        ),
        ..default()
    });

    for index in 0..3 {
        commands.spawn((
            ButtonBundle {
                image: UiImage {
                    texture: texture_handle.clone(),
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(player_transform.translation.y - 200.0),
                        left: Val::Px(
                            player_transform.translation.x - 100.0 + ((index - 1) as f32) * 200.0,
                        ),
                        right: Val::Auto,
                        bottom: Val::Auto,
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(
                        Val::Px(CHOOSE_DICE_ENTITY_SIZE),
                        Val::Px(CHOOSE_DICE_ENTITY_SIZE),
                    ),
                    ..Style::DEFAULT
                },
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            },
            DiceButton {
                value: Modification::AutoAttack,
            },
        ));
    }
}

pub fn lvlup_dice_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<DiceButton>)>,
    mut modification_choose_event: EventWriter<ChooseModificationEvent>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                // TODO: fixed from state to events
                game_simulation_next_state.set(GameSimulationState::Running);
                app_state_next_state.set(AppState::Game);

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
    mut dice_buttons_query: Query<Entity, With<DiceButton>>,
) {
    for entity in dice_buttons_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
