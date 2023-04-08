use bevy::prelude::*;

use crate::main_menu::components::MainMenu;
use crate::AppState;

use super::components::*;
use super::constants::*;

pub fn play_button_interaction(
    mut game_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = NORMAL_BUTTON_COLOR.into();

        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into();
                game_state.set(AppState::Game);
            }
            Interaction::Hovered => {}
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    main_menu_setup(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn toggle_game_status(
    mut game_status_state: ResMut<NextState<AppState>>,
    simulation_state: Res<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let concrete_simultation_state = simulation_state.0;

        match concrete_simultation_state {
            AppState::Game => game_status_state.set(AppState::Game),
            AppState::MainMenu => game_status_state.set(AppState::MainMenu),
            AppState::GameOver => {}
        }
    }
}

pub fn main_menu_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                    ..default()
                },
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "NAZVANIE NE PRIDUMALI",
                                get_text_style(asset_server, 60.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Play Button
            parent
                .spawn((
                    ButtonBundle {
                        style: DEFAULT_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_text_style(asset_server, 30.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // Exit Button
            parent
                .spawn((
                    ButtonBundle {
                        style: DEFAULT_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Exit game",
                                get_text_style(asset_server, 30.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    return main_menu_entity;
}
