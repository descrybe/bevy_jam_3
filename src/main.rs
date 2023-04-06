use ::bevy::prelude::*;
use events::game_over::GameOver;
use resources::score::Score;
use systems::{
    camera::{camera_follow, spawn_camera},
    enemy::{change_enemy_direction, enemy_hit_player, enemy_movement, spawn_enemies},
    game::game_over_hander,
    player::{change_player_direction, player_movement, spawn_player},
};

mod components;
mod entities;
mod events;
mod resources;
mod services;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(enemy_movement)
        .add_system(enemy_hit_player)
        .add_system(camera_follow)
        .add_system(change_player_direction)
        .add_system(change_enemy_direction)
        .add_event::<GameOver>()
        .init_resource::<Score>()
        .add_system(game_over_hander)
        .run();
}
