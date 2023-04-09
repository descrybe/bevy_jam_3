use crate::game::player::components::Player;
use bevy::prelude::*;

use super::components::PlayerBinder;

pub fn update_bind(
    mut bind_query: Query<&mut Transform, With<PlayerBinder>>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerBinder>)>,
) {
    if bind_query.is_empty() || player_query.is_empty() {
        return;
    }

    for mut transform in bind_query.iter_mut() {
        let player_transform = player_query.get_single().unwrap();
        transform.translation.x = player_transform.translation.x;
        transform.translation.y = player_transform.translation.y;
    }
}
