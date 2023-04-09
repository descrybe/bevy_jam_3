use crate::game::player::{components::Player};
use bevy::prelude::*;

use super::components::SatteliteComponent;

pub fn update_sattelite(
    time: Res<Time>,
    mut sattelite_query: Query<(&mut SatteliteComponent, &mut Transform), With<SatteliteComponent>>,
    player_query: Query<&Transform, (With<Player>, Without<SatteliteComponent>)>,
) {
    if sattelite_query.is_empty() || player_query.is_empty() {
        return;
    }
    //TODO: change it a little bit. Maybe there is no way to apply this code to future sattelites
    // But right now it works with shurikens
    for (mut sattelite, mut transform) in sattelite_query.iter_mut() {
        let player_transform = player_query.get_single().unwrap();
        sattelite.angle += sattelite.speed as f32 * time.delta().as_secs_f32();
        let x = sattelite.radius * sattelite.angle.cos();
        let y = sattelite.radius * sattelite.angle.sin();
        transform.translation.x = player_transform.translation.x + x;
        transform.translation.y = player_transform.translation.y + y;
    }
}