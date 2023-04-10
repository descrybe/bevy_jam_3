use bevy::app::AppExit;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::events::*;
use crate::game::player::components::*;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let camera_position = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0);

    commands.spawn((
        Camera2dBundle {
            transform: camera_position,
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.4, 0.5, 0.3)),
                ..default()
            },
            tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
            ..default()
        },
        MainCamera {},
    ));
}

pub fn camera_follow(
    mut camera_query: Query<&mut Transform, (Without<Player>, With<MainCamera>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut camera = camera_query.single_mut();

    if let Ok(player) = player_query.get_single() {
        camera.translation.x = player.translation.x;
        camera.translation.y = player.translation.y;
    }
}
pub fn game_over_hander(mut game_over_event_writer: EventReader<GameOver>) {
    for _ in game_over_event_writer.iter() {
        println!("Game over!");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        app_exit_event_writer.send(AppExit);
    }
}
