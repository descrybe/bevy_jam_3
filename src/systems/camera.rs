use bevy::{
    prelude::{Camera2dBundle, Commands, Query, Transform, With, Without},
    utils::default,
    window::{PrimaryWindow, Window},
};

use crate::components::{main_camera::MainCamera, player::Player};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        MainCamera {},
    ));
}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<MainCamera>)>,
) {
    let mut camera = camera_query.single_mut();
    let player = player_query.single();

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
