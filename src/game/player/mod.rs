pub mod components;
pub mod resources;
mod systems;

use systems::*;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player).add_systems(
            (player_movement, change_player_direction, enemy_hit_player)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
