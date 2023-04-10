use bevy::prelude::*;

use super::components::{DiceButton, LvlUpHolder, LvlUpText};
use crate::{
    assets_cache::resources::AssetsCache,
    dice::resources::{DiceRoller, DiceService},
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

pub fn spawn_lvlup_dices(
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut dice_service: ResMut<DiceService>,
    asset_cache: Res<AssetsCache>,
) {
    let player_transform = player_query.get_single().unwrap();
    let font = asset_server.load("fonts/CyrillicPixel.ttf");
    let text_alignment = TextAlignment::Center;

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 50.0,
        color: Color::WHITE,
    };

    let dices_cache = &asset_cache.sprites.ui.dices;
    let dice_images: Vec<Handle<Image>> = vec![
        dices_cache.bottle.clone(),
        dices_cache.heart.clone(),
        dices_cache.radiance.clone(),
        dices_cache.flame.clone(),
        dices_cache.shuriken.clone(),
        dices_cache.lightning.clone(),
    ];

    let modifications = vec![
        Modification::AutoAttack,
        Modification::Health,
        Modification::Radiance,
        Modification::Splash,
        Modification::Stars,
        Modification::Lightning,
    ];

    let rolled_values_u16 = dice_service
        .roll_few_times(crate::dice::resources::EDice::D6, 3)
        .unwrap();

    let roll_result = rolled_values_u16
        .iter()
        .map(|result| usize::try_from(*result).unwrap());

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Choose modification!", text_style.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_xyz(
                player_transform.translation.x,
                player_transform.translation.y + 130.0,
                10.0,
            ),
            ..default()
        },
        LvlUpText {},
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            LvlUpHolder {},
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|column| {
                    for roll in roll_result {
                        column.spawn((
                            ButtonBundle {
                                image: UiImage::new(dice_images[roll - 1].clone()),
                                style: Style {
                                    margin: UiRect {
                                        left: Val::Px(5.0),
                                        right: Val::Px(5.0),
                                        ..default()
                                    },
                                    size: Size::new(
                                        Val::Px(DICE_DIMENTION_SPRITE_SIZE),
                                        Val::Px(DICE_DIMENTION_SPRITE_SIZE),
                                    ),
                                    ..default()
                                },

                                ..default()
                            },
                            DiceButton {
                                value: modifications[roll - 1],
                            },
                        ));
                    }
                });
        });
}

pub fn lvlup_dice_interaction(
    mut interaction_query: Query<
        (&Interaction, &DiceButton),
        (Changed<Interaction>, With<DiceButton>),
    >,
    mut modification_choose_event: EventWriter<ChooseModificationEvent>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>,
) {
    for (interaction, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_simulation_next_state.set(GameSimulationState::Running);
                app_state_next_state.set(AppState::Game);

                modification_choose_event.send(ChooseModificationEvent {
                    modification: button.value,
                });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn despawn_lvlup_dices(
    mut commands: Commands,
    mut dice_buttons_query: Query<Entity, With<LvlUpHolder>>,
    lvl_up_text_query: Query<Entity, With<LvlUpText>>,
) {
    for entity in dice_buttons_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    for lvl_up_text in lvl_up_text_query.iter() {
        commands.entity(lvl_up_text).despawn();
    }
}
