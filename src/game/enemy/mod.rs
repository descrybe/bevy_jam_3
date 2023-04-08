pub mod components;
pub mod resources;
mod systems;

use systems::*;

use crate::AppState;

use bevy::prelude::*;

use super::GameSimulationState;

pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_COUNT: usize = 4;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemies).add_systems(
            (enemy_movement, change_enemy_direction)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
        // .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
