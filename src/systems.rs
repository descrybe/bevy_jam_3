use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::events::*;
use crate::game::player::components::*;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let camera_position = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);

    commands.spawn((
        Camera2dBundle {
            transform: camera_position,
            ..default()
        },
        MainCamera {},
    ));
}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<MainCamera>)>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut camera = camera_query.single_mut();
    let player = player_query.single();

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

pub fn set_game_active(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            app_state_next_state.set(AppState::Game);
            println!("AppState::Game");
        }
    }
}

pub fn set_menu_active(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            app_state_next_state.set(AppState::MainMenu);
            println!("AppState::MainMenu");
        }
    }
}

pub fn game_over_hander(mut game_over_event_writer: EventReader<GameOver>) {
    for event in game_over_event_writer.iter() {
        println!("Game over!");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        app_exit_event_writer.send(AppExit);
    }
}
