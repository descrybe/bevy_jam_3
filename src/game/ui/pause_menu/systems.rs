use bevy::app::AppExit;
use bevy::prelude::*;

use crate::main_menu::constants::{
    get_text_style, DEFAULT_BUTTON_STYLE, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
};
use crate::AppState;

use super::components::*;

pub fn resume_button_interaction(
    mut game_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
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

pub fn exit_button_interaction(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitButtonFromPauseMenu>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = NORMAL_BUTTON_COLOR.into();

        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {}
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    pause_menu_setup(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
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
            AppState::GameOver => game_status_state.set(AppState::GameOver),
            AppState::LvlUp => game_status_state.set(AppState::LvlUp),
            AppState::PauseMenu => game_status_state.set(AppState::PauseMenu),
        }
    }
}

pub fn pause_menu_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let texture_handle = asset_server.load("sprites/button.png");

    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    gap: Size::new(Val::Px(8.0), Val::Px(15.0)),
                    ..default()
                },
                background_color: Color::rgb(0.0, 0.5, 0.5).into(),
                transform: Transform::from_xyz(0.0, 0.0, 10.0),
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            // Resume Button
            parent
                .spawn((
                    ButtonBundle {
                        image: UiImage {
                            texture: texture_handle.clone(),
                            ..default()
                        },
                        style: DEFAULT_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    ResumeButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Resume",
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
                        image: UiImage {
                            texture: texture_handle.clone(),
                            ..default()
                        },
                        style: DEFAULT_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    ExitButtonFromPauseMenu {},
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

    return pause_menu_entity;
}

pub fn set_pause_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if app_state.0 != AppState::Game {
            app_state_next_state.set(AppState::Game);
        } else if app_state.0 != AppState::PauseMenu {
            app_state_next_state.set(AppState::PauseMenu);
        }
    }
}
