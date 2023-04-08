pub mod components;
mod events;
mod resources;
mod systems;

use systems::*;

use crate::AppState;

use bevy::prelude::*;

use self::{events::WaveSpawnEvent, resources::EnemyWavesSpawnConfig};

use super::GameSimulationState;

pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_COUNT: usize = 5;

const WAVE_SPAWN_DELAY: f32 = 2.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
        app.add_systems(
            (
                enemy_movement,
                wave_timer_tracking_system,
                spawn_enemie_wave,
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        )
        .add_event::<WaveSpawnEvent>()
        .insert_resource(EnemyWavesSpawnConfig {
            timer: Timer::from_seconds(WAVE_SPAWN_DELAY, TimerMode::Repeating),
        });
    }
}
