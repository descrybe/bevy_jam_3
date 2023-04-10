use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_kira_audio::{Audio, AudioControl};

use crate::main_menu::components::MainMenu;
use crate::AppState;

use super::components::*;
use super::constants::*;

pub fn play_button_interaction(
    mut game_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
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
        (Changed<Interaction>, With<ExitButton>),
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

pub fn settings_button_interaction(
    mut game_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = NORMAL_BUTTON_COLOR.into();

        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into();
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
            AppState::GameOver => game_status_state.set(AppState::GameOver),
            AppState::LvlUp => game_status_state.set(AppState::LvlUp),
            AppState::PauseMenu => game_status_state.set(AppState::PauseMenu),
        }
    }
}

pub fn main_menu_setup(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let texture_handle = asset_server.load("sprites/button.png");
    let main_menu_entity = commands
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
                                "SIDE EFFECT SOMETHING",
                                get_text_style(asset_server, 60.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        style: Style {
                            margin: UiRect {
                                bottom: Val::Px(30.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    });
                });
            // Play Button
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
            // Settings Button
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
                    SettingsButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Settings",
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
                    ExitButton {},
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

#[derive(Component)]
pub struct MenuBackground;

pub fn spawn_menu_bg(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let texture_handle = asset_server.load("sprites/menu_background.png");
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            texture: texture_handle.into(),
            sprite: Sprite {
                custom_size: Option::Some(Vec2::new(window.width(), window.height())),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -1.0),
            ..default()
        },
        MenuBackground {}
    ));
}

pub fn despawn_menu_bg(mut commands: Commands, mut bg_query: Query<Entity, With<MenuBackground>>) {
    let menu_bg = bg_query.get_single().unwrap();
    commands.entity(menu_bg).despawn();
}


pub fn start_menu_audio(
    app_state: Res<State<AppState>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let audio_track = asset_server.load("audio/main_menu_theme.mp3");

    println!("app_state.0 {:?}", app_state.0);

    if app_state.0 == AppState::MainMenu {
        audio.play(audio_track).looped();
    } else if app_state.0 == AppState::Game {
        audio.pause();
    }
}
